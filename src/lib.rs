pub mod prelude;
pub use prelude::investment_api;
use std::time::Duration;
use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::service::interceptor::InterceptedService;
use tonic::transport::{Channel, ClientTlsConfig};
use tonic::Request;

use investment_api::users_service_client::UsersServiceClient;


pub const PROD_ENDPOINT: &'static str = "https://invest-public-api.tinkoff.ru:443";
pub const SANDBOX_ENDPOINT: &'static str = "https://sandbox-invest-public-api.tinkoff.ru:443";


pub struct ServiceConfig {
    pub base_url: String,
    pub auth_token: String,
    pub user_agent: Option<String>,
    pub headers: Option<(&'static str, String)>,
}

pub struct Ready;
pub struct NotReady;

pub struct GrpcClientFactory<State = NotReady> {
    base_url: String,
    auth_token: String,
    tls_config: ClientTlsConfig,
    channel: Option<Channel>,
    metadata: MetadataMap,
    state: std::marker::PhantomData<State>
}

impl GrpcClientFactory<NotReady> {
    pub fn new(base_url: String, auth_token: String) -> GrpcClientFactory<NotReady> {
        let tls_config = ClientTlsConfig::new().with_native_roots();
        Self {
            base_url,
            auth_token,
            tls_config,
            channel: None,
            metadata: MetadataMap::new(),
            state: std::marker::PhantomData::<NotReady>,
        }
    }

    pub async fn connect(
        self,
        timeout: Duration,
        rate_limit: (u64, Duration),
    ) -> Result<GrpcClientFactory<Ready>, Box<dyn std::error::Error>> {
        // let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse()?;
        let channel = Channel::from_shared(self.base_url.clone())?
            .tls_config(self.tls_config.clone())?
            .rate_limit(rate_limit.0, rate_limit.1)
            .timeout(timeout)
            .connect()
            .await?;
        Ok(GrpcClientFactory {
            base_url: self.base_url,
            auth_token: self.auth_token,
            tls_config: self.tls_config,
            metadata: self.metadata,
            channel: Some(channel),
            state: std::marker::PhantomData::<Ready>,
        })
        // self.channel = Some(channel);

        // Ok(self.
    }


    pub async fn user_service(&self, channel: Channel) -> UsersServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
         UsersServiceClient::with_interceptor(channel.clone(), move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }

}

impl GrpcClientFactory<Ready> {
    pub fn user_service(&self) -> UsersServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
        let channel = self.channel.clone();
         UsersServiceClient::with_interceptor(channel.unwrap(), move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
        
    }
}
