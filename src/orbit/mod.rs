use async_trait::async_trait;
use sqlx::mysql::{MySqlPool, MySqlRow, MySqlArguments};
use sqlx::{FromRow, Arguments, Execute};

pub mod builder;
pub mod schema;

#[async_trait]
pub trait Orbit: Sized + Send + Unpin + for<'r> FromRow<'r, MySqlRow> {
    fn table_name() -> &'static str;

    // Optional: Override this if your primary key is not "id"
    fn primary_key() -> &'static str {
        "id"
    }

    // Required to support update/delete on instance
    fn id(&self) -> i64;

    // Lifecycle hooks
    fn boot() {}

    fn query() -> builder::Builder<Self> {
        builder::Builder::new(Self::table_name())
    }

    async fn all(pool: &MySqlPool) -> Result<Vec<Self>, sqlx::Error> {
        Self::query().get(pool).await
    }

    async fn find(pool: &MySqlPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        Self::query().where_eq(Self::primary_key(), id).first(pool).await
    }

    async fn create<D>(pool: &MySqlPool, data: D) -> Result<u64, sqlx::Error>
    where D: serde::Serialize + Send + Sync
    {
        let value = serde_json::to_value(data).unwrap();
        let object = value.as_object().expect("Data must be an object");

        let mut keys = Vec::new();
        let mut placeholders = Vec::new();
        let mut args = MySqlArguments::default();

        for (k, v) in object {
            keys.push(k.clone());
            placeholders.push("?");

            match v {
                serde_json::Value::String(s) => args.add(s),
                serde_json::Value::Number(n) => {
                    if n.is_i64() { args.add(n.as_i64()); }
                    else if n.is_u64() { args.add(n.as_u64()); }
                    else if n.is_f64() { args.add(n.as_f64()); }
                },
                serde_json::Value::Bool(b) => args.add(b),
                serde_json::Value::Null => args.add(Option::<String>::None),
                _ => args.add(v.to_string()),
            }
        }

        let sql = format!("INSERT INTO {} ({}) VALUES ({})",
            Self::table_name(),
            keys.join(", "),
            placeholders.join(", ")
        );

        let res = sqlx::query_with(&sql, args).execute(pool).await?;
        Ok(res.last_insert_id())
    }

    async fn update<D>(&self, pool: &MySqlPool, data: D) -> Result<u64, sqlx::Error>
    where D: serde::Serialize + Send + Sync
    {
        let value = serde_json::to_value(data).unwrap();
        let object = value.as_object().expect("Data must be an object");

        let mut updates = Vec::new();
        let mut args = MySqlArguments::default();

        for (k, v) in object {
            updates.push(format!("{} = ?", k));
            match v {
                serde_json::Value::String(s) => args.add(s),
                serde_json::Value::Number(n) => {
                    if n.is_i64() { args.add(n.as_i64()); }
                    else if n.is_u64() { args.add(n.as_u64()); }
                    else if n.is_f64() { args.add(n.as_f64()); }
                },
                serde_json::Value::Bool(b) => args.add(b),
                serde_json::Value::Null => args.add(Option::<String>::None),
                _ => args.add(v.to_string()),
            }
        }

        // Add ID for WHERE clause
        args.add(self.id());

        let sql = format!("UPDATE {} SET {} WHERE {} = ?",
            Self::table_name(),
            updates.join(", "),
            Self::primary_key()
        );

        let res = sqlx::query_with(&sql, args).execute(pool).await?;
        Ok(res.rows_affected())
    }

    async fn delete(&self, pool: &MySqlPool) -> Result<u64, sqlx::Error> {
        let sql = format!("DELETE FROM {} WHERE {} = ?", Self::table_name(), Self::primary_key());
        let res = sqlx::query(&sql)
            .bind(self.id())
            .execute(pool)
            .await?;
        Ok(res.rows_affected())
    }

    // Relationships

    /// Has Many Relationship
    /// Example: User has many Posts
    /// user.has_many::<Post>("user_id")
    fn has_many<R>(&self, foreign_key: &str) -> builder::Builder<R>
    where R: Orbit + Send + Unpin + for<'r> FromRow<'r, MySqlRow>
    {
        R::query().where_eq(foreign_key, self.id())
    }

    /// Belongs To Relationship
    /// Example: Post belongs to User
    /// post.belongs_to::<User>(&pool, post.user_id).await
    async fn belongs_to<R>(pool: &MySqlPool, foreign_key_value: i64) -> Result<Option<R>, sqlx::Error>
    where R: Orbit + Send + Unpin + for<'r> FromRow<'r, MySqlRow>
    {
        R::find(pool, foreign_key_value).await
    }
}
