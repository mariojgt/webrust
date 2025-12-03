use async_trait::async_trait;
use crate::database::{Db, DbArguments, DbRow, DbPool, DatabaseManager};
use sqlx::{FromRow, Arguments, Execute};

pub mod builder;
pub mod schema;

#[async_trait]
pub trait Orbit: Sized + Send + Unpin + for<'r> FromRow<'r, DbRow> {
    fn table_name() -> &'static str;

    // Optional: Override this if your primary key is not "id"
    fn primary_key() -> &'static str {
        "id"
    }

    // Optional: Override this to specify a connection name
    fn connection() -> Option<&'static str> {
        None
    }

    // Configuration
    const TIMESTAMPS: bool = true;
    const SOFT_DELETES: bool = false;

    // Required to support update/delete on instance
    fn id(&self) -> i64;

    // Lifecycle hooks
    fn boot() {}

    fn query() -> builder::Builder<Self> {
        let mut builder = builder::Builder::new(Self::table_name());
        if Self::SOFT_DELETES {
            builder = builder.where_null("deleted_at");
        }
        builder
    }

    fn with_trashed() -> builder::Builder<Self> {
        builder::Builder::new(Self::table_name())
    }

    async fn all(manager: &DatabaseManager) -> Result<Vec<Self>, sqlx::Error> {
        Self::query().get(manager).await
    }

    async fn find(manager: &DatabaseManager, id: i64) -> Result<Option<Self>, sqlx::Error> {
        Self::query().where_eq(Self::primary_key(), id).first(manager).await
    }

    async fn find_or_fail(manager: &DatabaseManager, id: i64) -> Result<Self, sqlx::Error> {
        match Self::find(manager, id).await? {
            Some(model) => Ok(model),
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    async fn create<D>(manager: &DatabaseManager, data: D) -> Result<u64, sqlx::Error>
    where D: serde::Serialize + Send + Sync
    {
        let pool = manager.connection(Self::connection()).ok_or(sqlx::Error::Configuration("No database connection found".into()))?;
        let mut value = serde_json::to_value(data).unwrap();

        if let Some(obj) = value.as_object_mut() {
            if Self::TIMESTAMPS {
                let now = chrono::Local::now().naive_local().to_string();
                obj.insert("created_at".to_string(), serde_json::Value::String(now.clone()));
                obj.insert("updated_at".to_string(), serde_json::Value::String(now));
            }
        }

        let object = value.as_object().expect("Data must be an object");

        let mut keys = Vec::new();
        let mut placeholders = Vec::new();
        let mut args = DbArguments::default();

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

    async fn update<D>(&self, manager: &DatabaseManager, data: D) -> Result<u64, sqlx::Error>
    where D: serde::Serialize + Send + Sync
    {
        let pool = manager.connection(Self::connection()).ok_or(sqlx::Error::Configuration("No database connection found".into()))?;
        let mut value = serde_json::to_value(data).unwrap();

        if let Some(obj) = value.as_object_mut() {
            if Self::TIMESTAMPS {
                let now = chrono::Local::now().naive_local().to_string();
                obj.insert("updated_at".to_string(), serde_json::Value::String(now));
            }
        }

        let object = value.as_object().expect("Data must be an object");

        let mut updates = Vec::new();
        let mut args = DbArguments::default();

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

    async fn delete(&self, manager: &DatabaseManager) -> Result<u64, sqlx::Error> {
        let pool = manager.connection(Self::connection()).ok_or(sqlx::Error::Configuration("No database connection found".into()))?;

        if Self::SOFT_DELETES {
            let now = chrono::Local::now().naive_local().to_string();
            let sql = format!("UPDATE {} SET deleted_at = ? WHERE {} = ?", Self::table_name(), Self::primary_key());
            let res = sqlx::query(&sql)
                .bind(now)
                .bind(self.id())
                .execute(pool)
                .await?;
            Ok(res.rows_affected())
        } else {
            let sql = format!("DELETE FROM {} WHERE {} = ?", Self::table_name(), Self::primary_key());
            let res = sqlx::query(&sql)
                .bind(self.id())
                .execute(pool)
                .await?;
            Ok(res.rows_affected())
        }
    }

    async fn force_delete(&self, manager: &DatabaseManager) -> Result<u64, sqlx::Error> {
        let pool = manager.connection(Self::connection()).ok_or(sqlx::Error::Configuration("No database connection found".into()))?;
        let sql = format!("DELETE FROM {} WHERE {} = ?", Self::table_name(), Self::primary_key());
        let res = sqlx::query(&sql)
            .bind(self.id())
            .execute(pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn restore(&self, manager: &DatabaseManager) -> Result<u64, sqlx::Error> {
        if !Self::SOFT_DELETES {
            return Ok(0);
        }
        let pool = manager.connection(Self::connection()).ok_or(sqlx::Error::Configuration("No database connection found".into()))?;
        let sql = format!("UPDATE {} SET deleted_at = NULL WHERE {} = ?", Self::table_name(), Self::primary_key());
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
    where R: Orbit + Send + Unpin + for<'r> FromRow<'r, DbRow>
    {
        R::query().where_eq(foreign_key, self.id())
    }

    /// Belongs To Relationship
    /// Example: Post belongs to User
    /// post.belongs_to::<User>(&pool, post.user_id).await
    async fn belongs_to<R>(manager: &DatabaseManager, foreign_key_value: i64) -> Result<Option<R>, sqlx::Error>
    where R: Orbit + Send + Unpin + for<'r> FromRow<'r, DbRow>
    {
        R::find(manager, foreign_key_value).await
    }

    /// Has One Relationship
    /// Example: User has one Profile
    fn has_one<R>(&self, foreign_key: &str) -> builder::Builder<R>
    where R: Orbit + Send + Unpin + for<'r> FromRow<'r, DbRow>
    {
        R::query().where_eq(foreign_key, self.id())
    }

    /// Belongs To Many Relationship (Many-to-Many)
    /// Example: User belongs to many Roles
    /// user.belongs_to_many::<Role>("role_user", "user_id", "role_id")
    fn belongs_to_many<R>(&self, pivot_table: &str, foreign_key: &str, related_key: &str) -> builder::Builder<R>
    where R: Orbit + Send + Unpin + for<'r> FromRow<'r, DbRow>
    {
        let related_table = R::table_name();
        let related_pk = R::primary_key(); // usually "id"

        // SELECT roles.* FROM roles
        // INNER JOIN role_user ON roles.id = role_user.role_id
        // WHERE role_user.user_id = ?
        R::query()
            .select(&[&format!("{}.*", related_table)])
            .join(pivot_table, &format!("{}.{}", related_table, related_pk), "=", &format!("{}.{}", pivot_table, related_key))
            .where_eq(&format!("{}.{}", pivot_table, foreign_key), self.id())
    }

    /// Morph One Relationship
    /// Example: Post has one Image
    /// post.morph_one::<Image>("imageable_id", "imageable_type")
    fn morph_one<R>(&self, id_column: &str, type_column: &str) -> builder::Builder<R>
    where R: Orbit + Send + Unpin + for<'r> FromRow<'r, DbRow>
    {
        R::query()
            .where_eq(id_column, self.id())
            .where_eq(type_column, Self::table_name())
    }

    /// Morph Many Relationship
    /// Example: Post has many Comments
    /// post.morph_many::<Comment>("commentable_id", "commentable_type")
    fn morph_many<R>(&self, id_column: &str, type_column: &str) -> builder::Builder<R>
    where R: Orbit + Send + Unpin + for<'r> FromRow<'r, DbRow>
    {
        R::query()
            .where_eq(id_column, self.id())
            .where_eq(type_column, Self::table_name())
    }
}
