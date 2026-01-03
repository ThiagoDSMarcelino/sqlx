use crate::app_config;

pub fn remove_connection(connection_name: &str) {
    match app_config::AppConfig::load() {
        Ok(mut config) => match config.remove_connection(connection_name) {
            Ok(_) => {
                config.save().unwrap_or_else(|err| {
                    eprintln!("Failed to save configuration: {}", err);
                });
                println!("Connection '{}' removed successfully.", connection_name);
            }
            Err(err) => {
                eprintln!("Failed to remove connection: {}", err);
                return;
            }
        },
        Err(err) => {
            eprintln!("Failed to load configuration: {}", err);
        }
    }
}
