use clap::Parser;

mod app_config;
mod cli;
mod drivers;

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Conn { cmd } => match cmd {
            cli::ConnCommands::Add {
                connection_name,
                connection_type,
                dns,
            } => {
                add_connection(&connection_name, &connection_type, &dns);
            }
            cli::ConnCommands::Remove { connection_name } => {
                remove_connection(&connection_name);
            }
        },
        cli::Commands::Run {
            file_path,
            connection_name,
        } => {
            run_sql_file(&file_path, &connection_name);
        }
    }
}

fn add_connection(connection_name: &str, connection_type: &str, dns: &str) {
    let driver = drivers::get_driver(&connection_type);

    if driver.is_none() {
        eprintln!("Error: Unsupported connection type '{}'.", connection_type);
        return;
    }

    if let Err(err) = driver.unwrap().test_connection(dns) {
        eprintln!("Connection test failed: {}", err);
        return;
    }

    match app_config::AppConfig::load() {
        Ok(mut config) => {
            match config.add_connection(connection_name.to_string(), dns.to_string()) {
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
            }
        }
        Err(err) => {
            eprintln!("Failed to load configuration: {}", err);
        }
    }
}

fn remove_connection(connection_name: &str) {
    println!("Removing connection: {}", connection_name);
}

fn run_sql_file(file_path: &str, connection_name: &str) {
    println!(
        "Running SQL file: {} on connection: {}",
        file_path, connection_name
    );
}
