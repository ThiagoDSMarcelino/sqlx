use clap::Subcommand;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = "Multi-database SQL CLI Tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Conn {
        #[command(subcommand)]
        cmd: ConnCommands,
    },
    Run {
        file_path: String,
        connection_name: String,
    },
}

#[derive(Subcommand)]
pub enum ConnCommands {
    Add {
        connection_name: String,
        connection_type: String,
        dns: String,
    },
    Remove {
        connection_name: String,
    },
    List,
}
