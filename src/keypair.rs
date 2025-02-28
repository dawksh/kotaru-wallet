use crate::utils::get_config_path;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::LocalSigner;
use eyre::Result;
use std::{
    fs::{read_to_string, write},
    str::FromStr,
};

use tokio::fs::read_to_string as tokio_read_to_string;

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
                    let rpc = "https://base.llamarpc.com".parse().unwrap();
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

pub fn store_keypair(name: &str, key: &str) {
    let config_path = get_config_path();
    let new_entry = format!("{}={}", name, key);

    let new_content = match read_to_string(&config_path) {
        Ok(data) => format!("{}\n{}", data.trim(), new_entry), // Append keypair
        Err(_) => new_entry, // If file doesn't exist, create a new one
    };

    if let Err(err) = write(&config_path, new_content) {
        println!("Failed to write file: {}", err);
    }
}
