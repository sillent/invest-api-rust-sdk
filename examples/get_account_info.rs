use std::{env, time::Duration};

use futures::stream::Scan;
use invest_api_rust_sdk::{ServiceClientFactory, ServiceConfig, PROD_ENDPOINT, SANDBOX_ENDPOINT};

#[tokio::main]
async fn main() {
	let token_string = env::var("TOKEN").expect("TOKEN environment");
	let timeout = Duration::from_secs(20);
	let rate = (100u64, Duration::from_secs(1));

	// let factory = invest_api_rust_sdk::ServiceClientFactory::new(invest_api_rust_sdk::SANDBOX_ENDPOINT, &token_string).connect(timeout, rate).await.unwrap();
	// let mut user_client = factory.users_service();
	// let mut order_client = factory.orders_service();
	// let resp_user = user_client.get_user_tariff(invest_api_rust_sdk::contracts::GetUserTariffRequest{}).await;
	//
	let sc = ServiceConfig::new(SANDBOX_ENDPOINT, &token_string);
	let factory = ServiceClientFactory::try_from(sc).unwrap();
	let mut usc = factory.users_service2(None, None).await.unwrap();
	let usc2 = factory.users_service2(None, None);


	let resp_user = usc.get_user_tariff(invest_api_rust_sdk::contracts::GetUserTariffRequest{}).await;
	// let resp_order = usc.get_user_tariff(invest_api_rust_sdk::contracts::GetOrdersRequest{account_id: String::from("09412be6-f7a3-45b0-b3c1-d83e0876c2af")}).await;
	println!("response = {:#?}", resp_user);
	// println!("response orders = {:#?}", resp_order);
}
