use std::str::FromStr;

use crate::Greeter;
use crate::Greeting;
use fuels::types::transaction::TxPolicies;
use fuels::types::AssetId;
use fuels::{
    accounts::wallet::WalletUnlocked,
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
    types::{Identity, SizedAsciiString},
};

// pub(crate) async fn constructor(contract: &Greeter<WalletUnlocked>) -> FuelCallResponse<()> {
//     contract.methods().constructor().call().await.unwrap()
// }

pub(crate) async fn set_greeting(
    contract: &Greeter<WalletUnlocked>,
    greeting: String,
) -> FuelCallResponse<()> {
    let stringFormatted: SizedAsciiString<8> = greeting.try_into().unwrap();

    let base_asset_id: AssetId =
        AssetId::from_str("0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07")
            .unwrap();

    println!("Setting greeting to: {}", stringFormatted.to_string());

    contract
        .methods()
        .set_greeting(stringFormatted)
        // .add_custom_asset(base_asset_id, 2000000, None)
        // .with_tx_policies(TxPolicies::default().with_max_fee(400000))
        .call()
        // .simulate()
        .await
        .unwrap()
}

pub(crate) async fn clear_greeting(contract: &Greeter<WalletUnlocked>) -> FuelCallResponse<()> {
    contract.methods().clear_greeting().call().await.unwrap()
}

pub(crate) async fn current_greeting(
    contract: &Greeter<WalletUnlocked>,
) -> FuelCallResponse<Option<Greeting>> {
    contract
        .methods()
        .current_greeting()
        .simulate()
        .await
        .unwrap()
}
