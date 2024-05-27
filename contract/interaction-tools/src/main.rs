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

    dbg!("getting the provider");
    let provider = Provider::connect("testnet.fuel.network").await.unwrap();
    dbg!("have the provider");

    let wallet = WalletUnlocked::new_from_mnemonic_phrase(&phrase, Some(provider)).unwrap();

    // Get the wallet address. Used later with the faucet
    dbg!(wallet.address().to_string());
    // From a string.
    let greeter_contract_id =
        ContractId::from_str(GREETER_CONTRACT).expect("failed to create ContractId from string");

    // let addressStr: String = wallet.address().to_string();
    let address_bits: Bits256 = Bits256::from_hex_str(wallet.address().hash.to_string().as_str())
        .expect("unable to convert address");
    // dbg!(wallet.address().hash.to_string().as_str());
    let contract_instance = Greeter::new(greeter_contract_id, wallet);

    dbg!("Getting current greeting");
    let result = current_greeting(&contract_instance).await.value;

    dbg!("current greeting", result);
    dbg!("update greeting");
    let set_greeting_result = set_greeting(&contract_instance, "test".to_string())
        .await
        .value;
    dbg!(set_greeting_result);
    //
    // // contract_instance.
    // //     .get_owner()
    // //     .await
    // //     .expect("failed to get owner");
    //
    Ok(())
}
