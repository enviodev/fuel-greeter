use crate::Greeter;
use crate::Greeting;
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
    contract
        .methods()
        .set_greeting(stringFormatted)
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
        .simulate()
        .await
        .unwrap()
}
