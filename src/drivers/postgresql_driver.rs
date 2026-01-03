use crate::drivers::{Driver, add_driver};
use postgres::{Client, NoTls};
use std::sync::Arc;

struct PostgreSQLDriver;

impl PostgreSQLDriver {
    fn new() -> Self {
        PostgreSQLDriver
    }
}

impl Driver for PostgreSQLDriver {
    fn test_connection(&self, dns: &str) -> Result<(), String> {
        match Client::connect(dns, NoTls) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}

pub fn register() {
    add_driver(
        &["pg", "postgres", "postgresql"],
        Arc::new(PostgreSQLDriver::new()),
    );
}
