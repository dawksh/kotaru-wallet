use crate::utils::{get_config_path, get_rpc_url};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::LocalSigner;
use eyre::Result;
use std::{fs::read_to_string, str::FromStr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::fs::{read_to_string as tokio_read_to_string, File};

pub fn get_wallets() {
    let config_path = get_config_path();
    match read_to_string(&config_path) {
        Ok(data) => {
            for line in data.lines() {
                if let Some((name, key)) = line.split_once("=") {
                    let signer = LocalSigner::from_str(key).unwrap();
                    println!("{} -> {}", name, signer.address());
                }
            }
        }
        Err(err) => {
            println!("Failed to read config. \n {}", err);
        }
    }
}

pub async fn get_balance() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    match tokio_read_to_string(&config_path).await {
        Ok(data) => {
            for line in data.lines() {
                if let Some((name, key)) = line.split_once("=") {
                    let signer = LocalSigner::from_str(key).unwrap();
                    let rpc = get_rpc_url();
                    let provider = ProviderBuilder::new().on_http(rpc);
                    let balance = provider.get_balance(signer.address()).await.unwrap();
                    println!("{} -> {} with {} ETH", name, signer.address(), balance);
                }
            }
        }
        Err(err) => {
            println!("Failed to read config. \n {}", err);
        }
    }
    Ok(())
}

pub async fn store_keypair(name: &str, key: &str) -> Result<(), tokio::io::Error> {
    // Get the path to the config file
    let config_path = crate::utils::get_config_path();
    let new_entry = format!("{}={}", name, key);

    let (mut file, new_content) = match File::open(&config_path).await {
        Ok(mut file) => {
            // Read the file to a buffer
            let mut content = String::new();
            file.read_to_string(&mut content).await?;

            // Trim the content and concatenate the new entry
            let new_content = format!("{}\n{}", content.trim(), new_entry);

            // return the file and the new content
            (file, new_content)
        }
        Err(err) if err.kind() == tokio::io::ErrorKind::NotFound => {
            // Create the file if it doesn't exist
            let file = File::create(&config_path).await?;

            // return the file and the new entry
            (file, new_entry) // Create new file
        }
        Err(err) => {
            panic!("Error occurred: {}", err);
        }
    };

    file.write_all(new_content.as_bytes()).await
}
