use std::env;

use invest_api_rust_sdk::{
    contracts::{
        users_service_client::UsersServiceClient, GetAccountsRequest, GetBankAccountsRequest,
        GetInfoRequest, GetMarginAttributesRequest, GetUserTariffRequest,
    },
    ServiceFactory, SANDBOX_ENDPOINT,
};
use tonic::{service::interceptor::InterceptedService, transport::Channel, Request, Status};

#[tokio::main]
async fn main() {
    let token_string = env::var("TOKEN").expect("TOKEN environment");
    let token_account_id = env::var("ACCOUNT_ID").unwrap_or_default();
    let factory = ServiceFactory::builder()
        .base_url(SANDBOX_ENDPOINT)
        .token(token_string)
        .user_agent("in-rust-we-trust")
        .build()
        .unwrap();
    let mut user_svc = factory.users_service();
    accounts(&mut user_svc).await;
    account_info(&mut user_svc).await;
    margin_attribute(&mut user_svc, token_account_id).await;
    user_tariff(&mut user_svc).await;
    bank_account(&mut user_svc).await;
}

async fn accounts(
    user_svc: &mut UsersServiceClient<
        InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, Status>>,
    >,
) {
    let acc_resp = user_svc
        .get_accounts(GetAccountsRequest { status: None })
        .await;
    println!("accounts: {acc_resp:#?}");
}

async fn account_info(
    user_svc: &mut UsersServiceClient<
        InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, Status>>,
    >,
) {
    let info_resp = user_svc.get_info(GetInfoRequest {}).await;
    println!("account info: {info_resp:#?}");
}

async fn margin_attribute(
    user_svc: &mut UsersServiceClient<
        InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, Status>>,
    >,
    account_id: String,
) {
    let info_resp = user_svc
        .get_margin_attributes(GetMarginAttributesRequest { account_id })
        .await;
    println!("margin attributes: {info_resp:#?}");
}

async fn user_tariff(
    user_svc: &mut UsersServiceClient<
        InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, Status>>,
    >,
) {
    let info_resp = user_svc.get_user_tariff(GetUserTariffRequest {}).await;
    println!("user tariff: {info_resp:#?}");
}

async fn bank_account(
    user_svc: &mut UsersServiceClient<
        InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, Status>>,
    >,
) {
    let info_resp = user_svc.get_bank_accounts(GetBankAccountsRequest {}).await;
    println!("user tariff: {info_resp:#?}");
}
