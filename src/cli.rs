use clap::Subcommand;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = "Multi-database SQL CLI Tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Manage database connections")]
    Conn {
        #[command(subcommand)]
        cmd: ConnCommands,
    },
    #[command(about = "Execute SQL commands")]
    Run {
        connection_name: String,
        file_path: String,
    },
    #[command(about = "List supported database drivers")]
    Drivers,
}

#[derive(Subcommand)]
pub enum ConnCommands {
    #[command(about = "Add a new database connection")]
    Add {
        connection_name: String,
        connection_driver: String,
        dns: String,
    },
    #[command(about = "Remove an existing database connection")]
    Remove { connection_name: String },
    #[command(about = "List all database connections")]
    List,
}
