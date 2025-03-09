use crate::utils::{get_config_path, get_rpc_url};
use alloy::network::{EthereumWallet, TransactionBuilder};
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::LocalSigner;
use eyre::Result;
use std::{fs::read_to_string, str::FromStr};

fn get_wallet_key(name: &str) -> String {
    let config_path = get_config_path();
    let data = read_to_string(&config_path).expect("Failed to read config file");

    for line in data.lines() {
        if let Some((wallet_name, key)) = line.split_once("=") {
            if wallet_name == name {
                return key.to_string();
            }
        }
    }

    panic!("Wallet name '{}' not found.", name);
}

pub async fn send_transaction(name: &str, amount: f64, to: &str) -> Result<()> {
    let key = get_wallet_key(name);
    let signer = LocalSigner::from_str(&key).unwrap();
    let rpc = get_rpc_url();
    let wallet = EthereumWallet::from(signer.clone());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(rpc);

    let balance = provider.get_balance(signer.address()).await?;
    let wei_value = (amount * 1e18_f64) as u128;
    let wei_amount = U256::from(wei_value);

    if balance < wei_amount {
        panic!("Low balance");
    }

    let receiver = Address::from_str(to)?;
    let nonce = provider
        .get_transaction_count(signer.address())
        .latest()
        .await?;
    let chain_id = provider.get_chain_id().await?;

    let tx = TransactionRequest::default()
        .with_to(receiver)
        .with_value(wei_amount)
        .with_nonce(nonce + 1)
        .with_chain_id(chain_id);

    let tx_hash = provider
        .send_transaction(tx)
        .await?
        .with_required_confirmations(2)
        .watch()
        .await?;

    println!("Transaction Executed with Hash: {}", tx_hash);

    Ok(())
}
