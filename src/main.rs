use clap::{Parser, Subcommand};
mod keypair;

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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, key } => {
            keypair::store_keypair(&name, &key);
        }
    }
}
