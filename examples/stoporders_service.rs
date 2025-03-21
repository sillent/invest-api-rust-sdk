use std::env;

use invest_api_rust_sdk::{contracts::StopOrderStatusOption, ServiceFactory, SANDBOX_ENDPOINT};

#[tokio::main]
async fn main() {
	let token_string = env::var("TOKEN").expect("TOKEN environment");
	let factory = ServiceFactory::builder()
		.base_url(SANDBOX_ENDPOINT)
		.token(token_string)
		.user_agent("in-rust-we-trust")
		.build()
		.unwrap();
	let mut stopord_svc = factory.stop_orders_service();
	let stopord_resp = stopord_svc.get_stop_orders(invest_api_rust_sdk::contracts::GetStopOrdersRequest{
		account_id: "...".to_owned(),
		status: StopOrderStatusOption::StopOrderStatusActive.into(),
		..Default::default()
	}).await;
	println!("{stopord_resp:#?}");
}

