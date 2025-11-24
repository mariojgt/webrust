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

    /// Paginate results (Laravel-style)
    /// Usage: query.paginate(manager, page, per_page).await
    pub async fn paginate(mut self, manager: &DatabaseManager, page: i64, per_page: i64) -> Result<(Vec<T>, i64), sqlx::Error> {
        let pool = manager.connection(T::connection()).ok_or(sqlx::Error::Configuration("No database connection found".into()))?;

        // Get total count
        let count_sql = format!("SELECT COUNT(*) as count FROM {}", self.table);
        let count_result: (i64,) = sqlx::query_as(&count_sql).fetch_one(pool).await?;
        let total = count_result.0;

        // Apply pagination
        self.offset = Some((page - 1) * per_page);
        self.limit = Some(per_page);

        let sql = self.to_sql();
        let args = self.build_arguments();
        let items = sqlx::query_as_with(&sql, args)
            .fetch_all(pool)
            .await?;

        Ok((items, total))
    }

    /// Get distinct results
    pub fn distinct(mut self) -> Self {
        if let Some(first) = self.select.first_mut() {
            if !first.starts_with("DISTINCT") {
                *first = format!("DISTINCT {}", first);
            }
        }
        self
    }

    /// Add OR WHERE clause
    pub fn or_where<V>(mut self, col: &str, op: &str, val: V) -> Self
    where V: 'static + Encode<'static, Db> + Type<Db> + Send + Sync + Clone
    {
        if !self.wheres.is_empty() {
            // Replace the last AND with OR by modifying how we construct the clause
            let last_where = self.wheres.pop().unwrap();
            self.wheres.push(format!("{} OR {} {} ?", last_where, col, op));
        } else {
            self.wheres.push(format!("{} {} ?", col, op));
        }
        self.argument_appliers.push(Box::new(move |args| {
            args.add(val.clone());
        }));
        self
    }

    /// Where IN clause
    pub fn where_in<V>(mut self, col: &str, values: Vec<V>) -> Self
    where V: 'static + Encode<'static, Db> + Type<Db> + Send + Sync + Clone
    {
        let placeholders = vec!["?"; values.len()].join(", ");
        self.wheres.push(format!("{} IN ({})", col, placeholders));

        for val in values {
            self.argument_appliers.push(Box::new(move |args| {
                args.add(val.clone());
            }));
        }
        self
    }

    /// Where NOT IN clause
    pub fn where_not_in<V>(mut self, col: &str, values: Vec<V>) -> Self
    where V: 'static + Encode<'static, Db> + Type<Db> + Send + Sync + Clone
    {
        let placeholders = vec!["?"; values.len()].join(", ");
        self.wheres.push(format!("{} NOT IN ({})", col, placeholders));

        for val in values {
            self.argument_appliers.push(Box::new(move |args| {
                args.add(val.clone());
            }));
        }
        self
    }

    /// Where NULL clause
    pub fn where_null(mut self, col: &str) -> Self {
        self.wheres.push(format!("{} IS NULL", col));
        self
    }

    /// Where NOT NULL clause
    pub fn where_not_null(mut self, col: &str) -> Self {
        self.wheres.push(format!("{} IS NOT NULL", col));
        self
    }

    /// Where BETWEEN clause
    pub fn where_between<V>(mut self, col: &str, min: V, max: V) -> Self
    where V: 'static + Encode<'static, Db> + Type<Db> + Send + Sync + Clone
    {
        self.wheres.push(format!("{} BETWEEN ? AND ?", col));
        self.argument_appliers.push(Box::new(move |args| {
            args.add(min.clone());
        }));
        self.argument_appliers.push(Box::new(move |args| {
            args.add(max.clone());
        }));
        self
    }

    /// Add DESC sorting (shortcut)
    pub fn latest(mut self, column: &str) -> Self {
        self.order.push(format!("{} DESC", column));
        self
    }

    /// Add ASC sorting (shortcut)
    pub fn oldest(mut self, column: &str) -> Self {
        self.order.push(format!("{} ASC", column));
        self
    }

    /// Group by clause
    pub fn group_by(mut self, columns: &[&str]) -> Self {
        for col in columns {
            self.wheres.push(format!("GROUP BY {}", col));
        }
        self
    }

    /// Add HAVING clause (for GROUP BY)
    pub fn having<V>(mut self, col: &str, op: &str, val: V) -> Self
    where V: 'static + Encode<'static, Db> + Type<Db> + Send + Sync + Clone
    {
        self.wheres.push(format!("HAVING {} {} ?", col, op));
        self.argument_appliers.push(Box::new(move |args| {
            args.add(val.clone());
        }));
        self
    }
}
