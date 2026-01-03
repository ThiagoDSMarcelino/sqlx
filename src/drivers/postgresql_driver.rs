use crate::drivers::{Driver, add_driver};
use chrono::{DateTime, Utc};
use postgres::{Client, Column, NoTls, Row};
use std::sync::Arc;
use tabled::{builder::Builder, settings::Style};
use uuid::Uuid;

struct PostgreSQLDriver;

impl PostgreSQLDriver {
    fn new() -> Self {
        PostgreSQLDriver
    }
}

impl Driver for PostgreSQLDriver {
    fn name(&self) -> &str {
        "PostgreSQL"
    }

    fn test_connection(&self, dns: &str) -> Result<(), String> {
        match Client::connect(dns, NoTls) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    fn execute_query(&self, dns: &str, query: &str) -> Result<String, String> {
        let mut client = Client::connect(dns, NoTls).map_err(|e| e.to_string())?;
        let rows = client.query(query, &[]).map_err(|e| e.to_string())?;

        if rows.is_empty() {
            return Ok("No rows returned".to_string());
        }

        let mut builder = Builder::default();

        let columns: Vec<String> = rows[0]
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect();

        builder.push_record(columns.clone());

        for row in rows {
            let mut row_values = Vec::new();
            for col in row.columns() {
                row_values.push(self.parse_pg_data_to_string(&row, col));
            }
            builder.push_record(row_values);
        }

        let mut table = builder.build();
        table.with(Style::modern());

        Ok(table.to_string())
    }
}

impl PostgreSQLDriver {
    fn parse_pg_data_to_string(&self, row: &Row, col: &Column) -> String {
        let col_id = col.name();

        match col.type_().name() {
            "int4" => {
                let value: Option<i32> = row.get(col_id);
                value
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "NULL".to_string())
            }
            "text" | "varchar" => {
                let value: Option<String> = row.get(col_id);
                value.unwrap_or_else(|| "NULL".to_string())
            }
            "uuid" => {
                let value: Option<Uuid> = row.get(col_id);
                value
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "NULL".to_string())
            }
            "timestamptz" => {
                let value: Option<DateTime<Utc>> = row.get(col_id);
                value
                    .map(|v| v.to_rfc3339())
                    .unwrap_or_else(|| "NULL".to_string())
            }
            _ => "UNSUPPORTED_TYPE".to_string(),
        }
    }
}

pub fn register() {
    add_driver(
        &["pg", "postgres", "postgresql"],
        Arc::new(PostgreSQLDriver::new()),
    );
}
