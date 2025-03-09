use core::panic;

use clap::{Parser, Subcommand};
mod keypair;
mod transactions;
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

    Get {
        #[arg(short)]
        balance: bool,
    },

    Send {
        #[arg(short)]
        value: f64,

        #[arg(short)]
        account: String,

        #[arg(short)]
        to: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, key } => {
            keypair::store_keypair(&name, &key);
        }

        Commands::Get { balance } => {
            if balance {
                match keypair::get_balance().await {
                    Err(_err) => {
                        panic!("Error occurred");
                    }
                    _ => {}
                }
            } else {
                keypair::get_wallets();
            }
        }

        Commands::Send { value, account, to } => {
            match transactions::send_transaction(&account, value, &to).await {
                Err(_err) => {
                    print!("{}", _err);
                    panic!("Error occurred");
                }
                _ => {}
            }
        }
    }
}
