use std::env;

use invest_api_rust_sdk::{ServiceFactory, SANDBOX_ENDPOINT};

#[tokio::main]
async fn main() {
	let token_string = env::var("TOKEN").expect("TOKEN environment");
	let factory =ServiceFactory::builder().base_url(SANDBOX_ENDPOINT).token(token_string).user_agent("test").build().unwrap();
	let mut usc = factory.users_service(None, None);
	let resp_user = usc.get_accounts(invest_api_rust_sdk::contracts::GetAccountsRequest{}).await;
	println!("{resp_user:#?}");
}
