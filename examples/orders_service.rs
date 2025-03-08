use std::env;

use invest_api_rust_sdk::{ServiceFactory, SANDBOX_ENDPOINT};

#[tokio::main]
async fn main() {
	let token_string = env::var("TOKEN").expect("TOKEN environment");
	let factory = ServiceFactory::builder()
		.base_url(SANDBOX_ENDPOINT)
		.token(token_string)
		.user_agent("in-rust-we-trust")
		.build()
		.unwrap();
	let mut orders_sv = factory.orders_service();
	let orders = orders_sv.get_orders(invest_api_rust_sdk::contracts::GetOrdersRequest{account_id: "xxx".to_owned()}).await;
	println!("{orders:#?}");
}
