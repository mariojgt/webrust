use crate::database::{Db, DbArguments, DbRow, DbPool, DatabaseManager};
use crate::orbit::Orbit;
use sqlx::{FromRow, Execute, Encode, Type, Arguments};
use std::marker::PhantomData;

/// The Query Builder
pub struct Builder<T> {
    table: String,
    select: Vec<String>,
    joins: Vec<String>,
    wheres: Vec<String>,
    order: Vec<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    argument_appliers: Vec<Box<dyn Fn(&mut DbArguments) + Send + Sync>>,
    _marker: PhantomData<T>,
}

impl<T> Builder<T>
where T: Orbit + Send + Unpin + for<'r> FromRow<'r, DbRow>
{
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            select: vec!["*".to_string()],
            joins: Vec::new(),
            wheres: Vec::new(),
            order: Vec::new(),
            limit: None,
            offset: None,
            argument_appliers: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn select(mut self, columns: &[&str]) -> Self {
        self.select = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Add a raw where clause (e.g. "id = ?")
    /// You must provide the value in the execute phase or use `where_eq`
    pub fn where_raw(mut self, clause: &str) -> Self {
        self.wheres.push(clause.to_string());
        self
    }

    pub fn r#where<V>(mut self, col: &str, op: &str, val: V) -> Self
    where V: 'static + Encode<'static, Db> + Type<Db> + Send + Sync + Clone
    {
        self.wheres.push(format!("{} {} ?", col, op));
        self.argument_appliers.push(Box::new(move |args| {
            args.add(val.clone());
        }));
        self
    }

    pub fn where_eq<V>(mut self, col: &str, val: V) -> Self
    where V: 'static + Encode<'static, Db> + Type<Db> + Send + Sync + Clone
    {
        self.r#where(col, "=", val)
    }

    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.order.push(format!("{} {}", column, direction));
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Add a JOIN clause
    /// join("posts", "users.id", "=", "posts.user_id")
    pub fn join(mut self, table: &str, first: &str, op: &str, second: &str) -> Self {
        self.joins.push(format!("INNER JOIN {} ON {} {} {}", table, first, op, second));
        self
    }

    /// Add a LEFT JOIN clause
    pub fn left_join(mut self, table: &str, first: &str, op: &str, second: &str) -> Self {
        self.joins.push(format!("LEFT JOIN {} ON {} {} {}", table, first, op, second));
        self
    }

    /// Add a WHERE EXISTS clause
    /// Useful for "where_has" type queries
    pub fn where_exists<R>(mut self, subquery: Builder<R>) -> Self 
    where R: Orbit + Send + Unpin + for<'r> FromRow<'r, DbRow>
    {
        let sql = subquery.to_sql();
        self.wheres.push(format!("EXISTS ({})", sql));
        self.argument_appliers.extend(subquery.argument_appliers);
        self
    }

    /// Dump the generated SQL to the console (debugging)
    pub fn dump(self) -> Self {
        let sql = self.to_sql();
        crate::dump!(sql);
        self
    }

    /// Dump the generated SQL and panic (debugging)
    pub fn dd(self) {
        let sql = self.to_sql();
        crate::dd!(sql);
    }

    pub fn to_sql(&self) -> String {
        let mut sql = format!("SELECT {} FROM {}", self.select.join(", "), self.table);

        if !self.joins.is_empty() {
            sql.push_str(" ");
            sql.push_str(&self.joins.join(" "));
        }

        if !self.wheres.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.wheres.join(" AND "));
        }

        if !self.order.is_empty() {
            sql.push_str(" ORDER BY ");
            sql.push_str(&self.order.join(", "));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        sql
    }

    fn build_arguments(&self) -> DbArguments {
        let mut args = DbArguments::default();
        for applier in &self.argument_appliers {
            applier(&mut args);
        }
        args
    }

    pub async fn get(self, manager: &DatabaseManager) -> Result<Vec<T>, sqlx::Error> {
        let pool = manager.connection(T::connection()).ok_or(sqlx::Error::Configuration("No database connection found".into()))?;
        let sql = self.to_sql();
        let args = self.build_arguments();
        sqlx::query_as_with(&sql, args)
            .fetch_all(pool)
            .await
    }

    pub async fn first(mut self, manager: &DatabaseManager) -> Result<Option<T>, sqlx::Error> {
        let pool = manager.connection(T::connection()).ok_or(sqlx::Error::Configuration("No database connection found".into()))?;
        self.limit = Some(1);
        let sql = self.to_sql();
        let args = self.build_arguments();
        sqlx::query_as_with(&sql, args)
            .fetch_optional(pool)
            .await
    }
}
