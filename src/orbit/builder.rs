use sqlx::mysql::{MySqlPool, MySqlRow, MySqlArguments};
use sqlx::{FromRow, Execute, MySql, Encode, Type, Arguments};
use std::marker::PhantomData;

/// The Query Builder
pub struct Builder<T> {
    table: String,
    select: Vec<String>,
    wheres: Vec<String>,
    order: Vec<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    arguments: MySqlArguments,
    _marker: PhantomData<T>,
}

impl<T> Builder<T>
where T: Send + Unpin + for<'r> FromRow<'r, MySqlRow>
{
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            select: vec!["*".to_string()],
            wheres: Vec::new(),
            order: Vec::new(),
            limit: None,
            offset: None,
            arguments: MySqlArguments::default(),
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
    where V: 'static + Encode<'static, MySql> + Type<MySql> + Send
    {
        self.wheres.push(format!("{} {} ?", col, op));
        self.arguments.add(val);
        self
    }

    pub fn where_eq<V>(mut self, col: &str, val: V) -> Self
    where V: 'static + Encode<'static, MySql> + Type<MySql> + Send
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

    pub fn to_sql(&self) -> String {
        let mut sql = format!("SELECT {} FROM {}", self.select.join(", "), self.table);

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

    pub async fn get(self, pool: &MySqlPool) -> Result<Vec<T>, sqlx::Error> {
        let sql = self.to_sql();
        sqlx::query_as_with(&sql, self.arguments)
            .fetch_all(pool)
            .await
    }

    pub async fn first(mut self, pool: &MySqlPool) -> Result<Option<T>, sqlx::Error> {
        self.limit = Some(1);
        let sql = self.to_sql();
        sqlx::query_as_with(&sql, self.arguments)
            .fetch_optional(pool)
            .await
    }
}
