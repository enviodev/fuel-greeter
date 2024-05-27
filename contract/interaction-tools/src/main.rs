extern crate dotenv;
use dotenv::dotenv;
use fuels::types::ContractId;
use fuels::{prelude::*, types::Bits256};
use std::env;
use std::str::FromStr;

use crate::contract_interfaces::greeter_contract::core::{current_greeting, set_greeting};

mod contract_interfaces;

// use crate::contract_interfaces::greeter_contract::info::{approval_weight, threshold};
//
// contracts:
/*
Greeter: 0x064c6e487c5eec390af0e1d68ff4ee4ef66f54bb749fc6928d46052e027b71f0
*/

abigen!(Contract(
    name = "Greeter",
    abi = "../sway-contract/abi.json"
),);
const GREETER_CONTRACT: &str = "0x064c6e487c5eec390af0e1d68ff4ee4ef66f54bb749fc6928d46052e027b71f0";

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv().ok();

    let phrase = env::var("MNEMONIC").expect("MNEMONIC must be set in .env");

    let provider = Provider::connect("beta-5.fuel.network").await.unwrap();

    let wallet = WalletUnlocked::new_from_mnemonic_phrase(&phrase, Some(provider.clone())).unwrap();

    let base_asset_id =
        AssetId::from_str("0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07")
            .unwrap();
    let balance = provider
        .get_asset_balance(wallet.address(), base_asset_id)
        .await?;
    let chain_id = provider.chain_id();

    dbg!(balance, chain_id);
    // call_handler.add_custom_asset(BASE_ASSET_ID, balance, None);

    // Get the wallet address. Used later with the faucet
    println!("Wallet address {}", wallet.address().to_string());
    // From a string.
    let greeter_contract_id =
        ContractId::from_str(GREETER_CONTRACT).expect("failed to create ContractId from string");

    // let addressStr: String = wallet.address().to_string();
    //
    let contract_instance = Greeter::new(greeter_contract_id, wallet);

    // // This also fails to read the value.
    // let result = current_greeting(&contract_instance).await.value;
    //
    println!("update greeting");
    let set_greeting_result = set_greeting(&contract_instance, "test    ".to_string())
        .await
        .value;
    dbg!(set_greeting_result);

    Ok(())
}
