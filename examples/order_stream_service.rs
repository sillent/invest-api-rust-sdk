use std::env;

use invest_api_rust_sdk::{contracts::OrderStateStreamRequest, ServiceFactory, SANDBOX_ENDPOINT};

#[tokio::main]
async fn main() {
    let token_string = env::var("TOKEN").expect("TOKEN environment");
    let account_id = env::var("ACCOUNT_ID").unwrap_or_default();
    let factory = ServiceFactory::builder()
        .base_url(SANDBOX_ENDPOINT)
        .token(token_string)
        .user_agent("in-rust-we-trust")
        .build()
        .unwrap();

    let mut oss = factory.orders_stream_service();

    let resp = oss
        .order_state_stream(OrderStateStreamRequest {
            accounts: vec![account_id],
            ping_delay_millis: Some(100),
        })
        .await;
    let mut k = resp.unwrap();
    loop {
        let k = k.get_mut().message().await.unwrap();
        println!("{k:#?}");
    }
}
