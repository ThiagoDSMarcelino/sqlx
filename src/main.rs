use clap::Parser;

mod cli;
mod drivers;

fn main() {
    let cli = cli::Cli::parse();

    drivers::add_driver("pg", "PostgreSQL Driver");   
    drivers::add_driver("postgres", "PostgreSQL Driver");   

    match cli.command {
        cli::Commands::Conn { cmd } => {
            match cmd {
                cli::ConnCommands::Add { connection_name, connection_type, dns } => {
                    let driver = drivers::get_driver(&connection_type);

                    if driver.is_none() {
                        eprintln!("Error: Unsupported connection type '{}'.", connection_type);
                        return;
                    }

                    println!("Adding connection: {}, Driver: {}, DNS: {}", connection_name, driver.unwrap(), dns);
                },
                cli::ConnCommands::Remove { connection_name } => {
                    println!("Removing connection: {}", connection_name);
                },
            }
        },
        cli::Commands::Run { file_path, connection_name } => {
            println!("Running SQL file: {} on connection: {}", file_path, connection_name);
        },
    }
}
