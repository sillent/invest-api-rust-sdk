use std::{env, time::Duration};

use invest_api_rust_sdk;


#[tokio::main]
async fn main() {
	let token_string = env::var("TOKEN").expect("TOKEN environment");
	let duration = Duration::from_secs(20);
	let rate = (100u64, Duration::from_secs(1));

	let factory = invest_api_rust_sdk::GrpcClientFactory::new(invest_api_rust_sdk::SANDBOX_ENDPOINT, &token_string).connect(duration, rate).await.unwrap();
	let mut user_client = factory.user_service();
	let resp = user_client.get_user_tariff(invest_api_rust_sdk::contracts::GetUserTariffRequest{}).await;
	println!("response = {:#?}", resp);
}
