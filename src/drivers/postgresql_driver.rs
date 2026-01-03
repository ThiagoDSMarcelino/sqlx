use crate::drivers::{add_driver, Driver};
use std::sync::Arc;

struct PostgreSQLDriver;

impl PostgreSQLDriver {
    pub fn new() -> Self {
        PostgreSQLDriver
    }
}

impl Driver for PostgreSQLDriver {
    fn name(&self) -> &str {
        "PostgreSQL Driver"
    }
}

pub fn register() {
    add_driver("pg", Arc::new(PostgreSQLDriver::new()));
}