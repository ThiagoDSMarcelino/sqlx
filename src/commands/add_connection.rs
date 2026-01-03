use crate::{app_config, drivers};

pub fn add_connection(connection_name: &str, connection_driver: &str, dns: &str) {
    let driver = drivers::get_driver(&connection_driver);

    if driver.is_none() {
        eprintln!(
            "Error: Unsupported connection driver '{}'.",
            connection_driver
        );
        return;
    }

    if let Err(err) = driver.unwrap().test_connection(dns) {
        eprintln!("Connection test failed: {}", err);
        return;
    }

    match app_config::AppConfig::load() {
        Ok(mut config) => match config.add_connection(connection_name, dns, connection_driver) {
            Ok(_) => {
                config.save().unwrap_or_else(|err| {
                    eprintln!("Failed to save configuration: {}", err);
                });
                println!("Connection '{}' added successfully.", connection_name);
            }
            Err(err) => {
                eprintln!("Failed to add connection: {}", err);
                return;
            }
        },
        Err(err) => {
            eprintln!("Failed to load configuration: {}", err);
        }
    }
}
