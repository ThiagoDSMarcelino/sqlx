use crate::{app_config, drivers};

pub fn run_sql_file(file_path: &str, connection_name: &str) {
    let config = match app_config::AppConfig::load() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Failed to load configuration: {}", err);
            return;
        }
    };

    let connection = match config.get_connection(connection_name) {
        Some(connection) => connection,
        None => {
            eprintln!("Connection '{}' not found.", connection_name);
            return;
        }
    };

    let driver = match drivers::get_driver(connection.driver()) {
        Some(driver) => driver,
        None => {
            eprintln!(
                "Unsupported connection driver for DNS '{}'.",
                connection.driver()
            );
            return;
        }
    };

    let sql_content = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read SQL file: {}", err);
            return;
        }
    };

    match driver.execute_query(connection.dns(), &sql_content) {
        Ok(result) => {
            println!("{}", result);
        }
        Err(err) => {
            eprintln!("Failed to execute SQL file: {}", err);
        }
    }
}
