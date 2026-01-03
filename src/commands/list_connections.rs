use crate::app_config;

pub fn list_connections() {
    match app_config::AppConfig::load() {
        Ok(config) => {
            let connections = config.get_connections();

            if connections.is_empty() {
                println!("No connections found.");
            } else {
                println!("Saved Connections:");
                for (name, connection) in connections.iter() {
                    println!(" - {}: {}", name, connection.dns());
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to load configuration: {}", err);
        }
    }
}
