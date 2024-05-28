use std::str::FromStr;

use crate::Greeter;
use crate::Greeting;
use fuels::accounts::provider::Provider;
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
const TEMP_GAS_LIMIT_HACK: u64 = 300000;

pub(crate) async fn set_greeting(
    contract: &Greeter<WalletUnlocked>,
    greeting: String,
    // provider: Provider,
) -> FuelCallResponse<()> {
    let stringFormatted: SizedAsciiString<8> = greeting.try_into().unwrap();
    //
    // let base_asset_id: AssetId =
    //     AssetId::from_str("0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07")
    //         .unwrap();

    // println!("Setting greeting to: {}", stringFormatted.to_string());
    // let gas_limit = provider.consensus_parameters().max_gas_per_tx() * 0.8;

    contract
        .methods()
        .set_greeting(stringFormatted)
        // .add_custom_asset(base_asset_id, 2000000, None)
        // .with_tx_policies(TxPolicies::default().with_max_fee(400000))
        .with_tx_policies(TxPolicies::default().with_script_gas_limit(TEMP_GAS_LIMIT_HACK))
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
        .with_tx_policies(TxPolicies::default().with_script_gas_limit(TEMP_GAS_LIMIT_HACK))
        .simulate()
        .await
        .unwrap()
}
