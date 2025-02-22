pub mod prelude;
use std::time;

pub use prelude::investment_api::*;

pub const PROD_ENDPOINT: &'static str = "https://invest-public-api.tinkoff.ru:443";
pub const SANDBOX_ENDPOINT: &'static str = "https://sandbox-invest-public-api.tinkoff.ru:443";


pub struct PoolServices {
    inner: tonic::transport::Endpoint,
}


impl PoolServices {
    pub fn new() -> PoolServices {
        let tls = tonic::transport::channel::ClientTlsConfig::new().with_native_roots();
        let endpoint = tonic::transport::Channel::from_static(SANDBOX_ENDPOINT)
            .tls_config(tls)
            .expect("tls configuration error")
            .rate_limit(200, time::Duration::from_secs(1));

        PoolServices { inner: endpoint }
    }

    pub async fn user_service(
        &self,
    ) -> orders_service_client::OrdersServiceClient<tonic::transport::Channel> {
        unimplemented!();
    }
}
