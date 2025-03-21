use std::{env, time::Duration};

use invest_api_rust_sdk::{contracts::{OrderDirection, OrderType, Quotation}, ServiceFactory, SANDBOX_ENDPOINT};

#[tokio::main]
async fn main() {
	let token_string = env::var("TOKEN").expect("TOKEN environment");
	let factory = ServiceFactory::builder()
		.base_url(SANDBOX_ENDPOINT)
		.token(token_string)
		.user_agent("in-rust-we-trust")
		.tcp_keepalive(Duration::from_millis(100))
		.build()
		.unwrap();
	let mut sandbox_svc = factory.sandbox_service();
	let account_resp = sandbox_svc.post_sandbox_order(invest_api_rust_sdk::contracts::PostOrderRequest{
		order_id: "...".to_owned(),
		quantity: 12,
		price: Some(Quotation{units: 1, nano: 1}),
		direction: OrderDirection::Buy.into(),
		account_id: "....".to_owned(),
		order_type: OrderType::Market.into(),
		instrument_id: "...".to_owned(),
		..Default::default()
	}).await;
	println!("{account_resp:#?}");
}

