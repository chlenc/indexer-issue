use crate::utils::print_title;
use fuels::accounts::wallet::WalletUnlocked;
use fuels::{
    prelude::{abigen, Bech32ContractId, Provider, TxParameters},
    types::ContractId,
};
use std::str::FromStr;

abigen!(Contract(name = "DApp", abi = "out/debug/contract-abi.json"));

const RPC: &str = "beta-4.fuel.network";
const CONTRACT_ADDRESS: &str = "0x08962250a6e4c0a862002aedc49a60dc6960826e309dcf3f32580b1d11744bf4";

#[tokio::test]
async fn main_test() {
    print_title("Main test");

    dotenv::dotenv().ok();

    //--------------- WALLETS ---------------
    let provider = Provider::connect(RPC).await.unwrap();

    let admin_pk = std::env::var("ADMIN").unwrap().parse().unwrap();
    let admin = WalletUnlocked::new_from_private_key(admin_pk, Some(provider.clone()));
    let contract_id: Bech32ContractId = ContractId::from_str(CONTRACT_ADDRESS).unwrap().into();
    let instance = DApp::new(contract_id, admin.clone());
    instance
        .methods()
        .test_function()
        .tx_params(TxParameters::default().with_gas_price(1))
        .call()
        .await
        .unwrap();
}
