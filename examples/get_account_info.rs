use std::env;

// use invest_api_rust_sdk::prelude::*;
use invest_api_rust_sdk;
use tonic::{metadata::MetadataValue, Request};

const SANDBOX_ADDR: &'static str = "https://sandbox-invest-public-api.tinkoff.ru:443";

#[tokio::main]
async fn main() {
	let token_string = env::var("TOKEN").unwrap_or("NO_TOKEN".to_owned());
	let tls =  tonic::transport::channel::ClientTlsConfig::new().with_native_roots();
	let channel = tonic::transport::Channel::from_static(SANDBOX_ADDR).tls_config(tls).unwrap().connect().await.expect("connected to endpoint");
	let token: MetadataValue<_> = format!("Bearer {}", token_string).parse().expect("token parsed as metadata");

	let mut user_client = invest_api_rust_sdk::users_service_client::UsersServiceClient::with_interceptor(channel.clone(), move |mut req: Request<()>| {
		req.metadata_mut().insert("authorization", token.clone());
		Ok(req)
	});
	let request = tonic::Request::new(invest_api_rust_sdk::GetAccountsRequest{});
	let response = user_client.get_accounts(request).await.expect("expected response");
	println!("response = {:?}", response);
}
