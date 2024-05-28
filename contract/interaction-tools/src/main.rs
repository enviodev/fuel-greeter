extern crate dotenv;
extern crate rand;

use dotenv::dotenv;
use fuels::prelude::*;
use fuels::types::ContractId;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::env;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

use crate::contract_interfaces::greeter_contract::core::clear_greeting;
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

const GREETINGS: [&str; 51] = [
    "Hello   ", "Hi      ", "Hey     ", "Greeting", "Salute  ", "Howdy   ", "Whats up", "Hola    ",
    "Bonjour ", "Ciao    ", "Salam   ", "Hallo   ", "Hei     ", "Aloha   ", "Namaste ", "Shalom  ",
    "Ahoy    ", "Yo      ", "Sup     ", "G'day   ", "Cheers  ", "Good day", "TopOfDaM", "GM      ",
    "GM GM   ", "GM GM GM", "Welcome ", "Peace   ", "HeyThere", "Hiya    ", "GM earth", "HowdyBro",
    "Hi World", "Hey you ", "Yo yo yo", "Hi folks", "HeyPeeps", "G-Morgan", "Howdy G ", "Salut   ",
    "HiFriend", "Hey Bud ", "GM buddy", "GudNight", "HeyBuddy", "Hows it ", "How you?", "WhatsNew",
    "LookHere", "LongTime", "Niiiiice",
];

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv().ok();

    let phrase = env::var("MNEMONIC").expect("MNEMONIC must be set in .env");

    let provider = Provider::connect("testnet.fuel.network").await.unwrap();

    let wallets = (0..10)
        .map(|i| {
            let path = format!("m/44'/1179993420'/0'/0/{}", i);
            WalletUnlocked::new_from_mnemonic_phrase_with_path(
                &phrase,
                Some(provider.clone()),
                &path,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    let base_asset_id =
        AssetId::from_str("0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07")
            .unwrap();

    for wallet in &wallets {
        let balance = provider
            .get_asset_balance(wallet.address(), base_asset_id)
            .await?;
        // let chain_id = provider.chain_id();

        if balance < 500000 {
            println!("Wallet address {}, with balance {}, can use the faucet here: https://faucet-testnet.fuel.network/?address={}", wallet.address(), balance, wallet.address());
            continue;
        }

        let greeter_contract_id = ContractId::from_str(GREETER_CONTRACT)
            .expect("failed to create ContractId from string");

        let contract_instance = Greeter::new(greeter_contract_id, wallet.clone());

        let result = current_greeting(&contract_instance).await.value;
        dbg!(result);

        let mut rng = thread_rng();
        if rng.gen_ratio(1, 6) {
            // EVery 1 in 6 times we clear the greeting
            println!("clear greeting");
            let clear_greeting_result = clear_greeting(&contract_instance).await;
            dbg!(clear_greeting_result);
        } else {
            let greeting = GREETINGS.choose(&mut rng).unwrap();
            println!("set greeting to {}", greeting);
            let set_greeting_result = set_greeting(&contract_instance, greeting.to_string()).await;
            dbg!(set_greeting_result);
        }

        sleep(Duration::from_secs(2)).await;
    }

    Ok(())
}
