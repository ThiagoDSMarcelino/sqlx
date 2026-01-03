use clap::Parser;

mod app_config;
mod cli;
mod connection;
mod drivers;
mod commands;

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Conn { cmd } => match cmd {
            cli::ConnCommands::Add {
                connection_name,
                connection_driver,
                dns,
            } => {
                commands::add_connection(&connection_name, &connection_driver, &dns);
            }
            cli::ConnCommands::Remove { connection_name } => {
                commands::remove_connection(&connection_name);
            }
            cli::ConnCommands::List => {
                commands::list_connections();
            }
        },
        cli::Commands::Run {
            file_path,
            connection_name,
        } => {
            commands::run_sql_file(&file_path, &connection_name);
        }
        cli::Commands::Drivers => {
            commands::list_drivers();
        }
    }
}
