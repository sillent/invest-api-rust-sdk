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
	let mut user_svc = factory.users_service();
	let account_resp = user_svc.get_accounts(invest_api_rust_sdk::contracts::GetAccountsRequest{status: None }).await;
	println!("{account_resp:#?}");
}
