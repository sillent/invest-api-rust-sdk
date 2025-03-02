use std::{env, time::Duration};

// use invest_api_rust_sdk::prelude::*;
use invest_api_rust_sdk;
// use tonic::{metadata::MetadataValue, Request};

const SANDBOX_ADDR: &'static str = "https://sandbox-invest-public-api.tinkoff.ru:443";

#[tokio::main]
async fn main() {
	let token_string = env::var("TOKEN").unwrap_or("NO_TOKEN".to_owned());
	let duration = Duration::from_secs(20);
	let rate = (100u64, Duration::from_secs(1));

	let factory = invest_api_rust_sdk::GrpcClientFactory::new(SANDBOX_ADDR.to_owned(), token_string).connect(duration, rate).await.unwrap();
	let mut user_client = factory.user_service();
	// let tls =  tonic::transport::channel::ClientTlsConfig::new().with_native_roots();
	// let channel = tonic::transport::Channel::from_static(SANDBOX_ADDR).tls_config(tls).unwrap().connect().await.expect("connected to endpoint");
	// let token: MetadataValue<_> = format!("Bearer {}", token_string).parse().expect("token parsed as metadata");

	// let mut user_client = invest_api_rust_sdk::users_service_client::UsersServiceClient::with_interceptor(channel.clone(), move |mut req: Request<()>| {
		// req.metadata_mut().insert("authorization", token.clone());
		// Ok(req)
	// });
	// let request = tonic::Request::new(invest_api_rust_sdk::investment_api::GetUserTariffRequest{});
	let resp = user_client.get_user_tariff(invest_api_rust_sdk::contracts::GetUserTariffRequest{}).await;
	// let response = user_client.get_accounts(request).await.expect("expected response");
	println!("response = {:#?}", resp);
}
