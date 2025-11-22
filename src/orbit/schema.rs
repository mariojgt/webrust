use std::fmt;

pub struct Schema;

impl Schema {
    /// Create a new table with a blueprint closure
    /// Note: In Rust we can't easily pass a closure that modifies a struct in the same way as PHP,
    /// but we can use a builder pattern.
    ///
    /// Usage:
    /// let sql = Schema::create("users", |table| {
    ///     table.id();
    ///     table.string("name");
    ///     table.string("email").unique();
    ///     table.timestamps();
    /// });
    pub fn create<F>(table_name: &str, callback: F) -> String
    where F: FnOnce(&mut Blueprint)
    {
        let mut blueprint = Blueprint::new(table_name);
        callback(&mut blueprint);
        blueprint.to_sql()
    }

    pub fn drop_if_exists(table_name: &str) -> String {
        format!("DROP TABLE IF EXISTS {};", table_name)
    }
}

pub struct Blueprint {
    table: String,
    columns: Vec<ColumnDefinition>,
    primary_key: Option<String>,
}

impl Blueprint {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            columns: Vec::new(),
            primary_key: None,
        }
    }

    pub fn id(&mut self) {
        self.columns.push(ColumnDefinition {
            name: "id".to_string(),
            data_type: "BIGINT".to_string(),
            auto_increment: true,
            nullable: false,
            unique: false,
            default: None,
        });
        self.primary_key = Some("id".to_string());
    }

    pub fn string(&mut self, name: &str) -> &mut ColumnDefinition {
        self.add_column(name, "VARCHAR(255)")
    }

    pub fn text(&mut self, name: &str) -> &mut ColumnDefinition {
        self.add_column(name, "TEXT")
    }

    pub fn integer(&mut self, name: &str) -> &mut ColumnDefinition {
        self.add_column(name, "INT")
    }

    pub fn boolean(&mut self, name: &str) -> &mut ColumnDefinition {
        self.add_column(name, "BOOLEAN")
    }

    pub fn timestamp(&mut self, name: &str) -> &mut ColumnDefinition {
        self.add_column(name, "DATETIME")
    }

    pub fn timestamps(&mut self) {
        self.timestamp("created_at").default("CURRENT_TIMESTAMP");
        self.timestamp("updated_at").default("CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP");
    }

    fn add_column(&mut self, name: &str, data_type: &str) -> &mut ColumnDefinition {
        let col = ColumnDefinition {
            name: name.to_string(),
            data_type: data_type.to_string(),
            auto_increment: false,
            nullable: false,
            unique: false,
            default: None,
        };
        self.columns.push(col);
        self.columns.last_mut().unwrap()
    }

    pub fn to_sql(&self) -> String {
        let mut lines = Vec::new();

        for col in &self.columns {
            lines.push(col.to_sql());
        }

        if let Some(pk) = &self.primary_key {
            lines.push(format!("PRIMARY KEY ({})", pk));
        }

        format!(
            "CREATE TABLE {} (\n    {}\n) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;",
            self.table,
            lines.join(",\n    ")
        )
    }
}

pub struct ColumnDefinition {
    name: String,
    data_type: String,
    auto_increment: bool,
    nullable: bool,
    unique: bool,
    default: Option<String>,
}

impl ColumnDefinition {
    pub fn nullable(&mut self) -> &mut Self {
        self.nullable = true;
        self
    }

    pub fn unique(&mut self) -> &mut Self {
        self.unique = true;
        self
    }

    pub fn default(&mut self, value: &str) -> &mut Self {
        self.default = Some(value.to_string());
        self
    }

    pub fn to_sql(&self) -> String {
        let mut parts = vec![self.name.clone(), self.data_type.clone()];

        if !self.nullable {
            parts.push("NOT NULL".to_string());
        }

        if self.auto_increment {
            parts.push("AUTO_INCREMENT".to_string());
        }

        if let Some(default) = &self.default {
            parts.push(format!("DEFAULT {}", default));
        }

        if self.unique {
            parts.push("UNIQUE".to_string());
        }

        parts.join(" ")
    }
}
