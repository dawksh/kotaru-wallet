use clap::{Parser, Subcommand};
mod keypair;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[arg(short, long)]
        name: String,

        #[arg(long)]
        key: String,
    },

    Get {},
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, key } => {
            keypair::store_keypair(&name, &key);
        }

        Commands::Get {} => {
            keypair::get_wallets();
        }
    }
}
