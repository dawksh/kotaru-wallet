use core::panic;

use clap::Parser;

use self::cli::{CliArgs, CliSubCommand};

mod cli;
mod keypair;
mod transactions;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let CliArgs { sub_command } = CliArgs::try_parse()?;

    match sub_command {
        CliSubCommand::Add { name, key } => {
            keypair::store_keypair(&name, &key).await?;
        }

        CliSubCommand::Get { balance } => {
            if balance {
                if let Err(_err) = keypair::get_balance().await {
                    panic!("Error occurred");
                }
            } else {
                keypair::get_wallets();
            }
        }

        CliSubCommand::Send { value, account, to } => {
            transactions::send_transaction(&account, value, &to)
                .await
                .map_err(|err| anyhow::anyhow!("Failed to send transaction: {}", err))?;
        }
    }

    Ok(())
}
