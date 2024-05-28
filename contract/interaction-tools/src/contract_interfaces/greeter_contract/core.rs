use crate::Greeter;
use crate::Greeting;
use fuels::types::transaction::TxPolicies;
use fuels::{
    accounts::wallet::WalletUnlocked, programs::call_response::FuelCallResponse,
    types::SizedAsciiString,
};

// pub(crate) async fn constructor(contract: &Greeter<WalletUnlocked>) -> FuelCallResponse<()> {
//     contract.methods().constructor().call().await.unwrap()
// }
//
const TEMP_GAS_LIMIT_HACK: u64 = 400000;

pub(crate) async fn set_greeting(
    contract: &Greeter<WalletUnlocked>,
    greeting: String,
) -> FuelCallResponse<()> {
    let string_formatted: SizedAsciiString<8> = greeting.try_into().unwrap();

    contract
        .methods()
        .set_greeting(string_formatted)
        .with_tx_policies(TxPolicies::default().with_script_gas_limit(TEMP_GAS_LIMIT_HACK))
        .call()
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
