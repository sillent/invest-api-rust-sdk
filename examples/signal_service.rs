use std::env;

use invest_api_rust_sdk::ServiceFactory;

#[tokio::main]
async fn main() {
    let token_string = env::var("TOKEN").expect("TOKEN environment");
    let factory = ServiceFactory::builder()
        .environment(invest_api_rust_sdk::Environment::Sandbox)
        .token(token_string)
        .user_agent("in-rust-we-trust")
        .build()
        .unwrap();
    let mut signal_svc = factory.signal_service();
    let signal_resp = signal_svc
        .get_signals(invest_api_rust_sdk::contracts::GetSignalsRequest {
            signal_id: None,
            strategy_id: None,
            strategy_type: None,
            instrument_uid: None,
            from: None,
            to: None,
            direction: None,
            active: None,
            paging: None,
        })
        .await;
    println!("{signal_resp:#?}");
}
