pub(crate) mod prelude;
pub use prelude::contracts;
use prelude::contracts::instruments_service_client::InstrumentsServiceClient;
use prelude::contracts::market_data_service_client::MarketDataServiceClient;
use prelude::contracts::market_data_stream_service_client::MarketDataStreamServiceClient;
use prelude::contracts::operations_service_client::OperationsServiceClient;
use prelude::contracts::operations_stream_service_client::OperationsStreamServiceClient;
use prelude::contracts::orders_service_client::OrdersServiceClient;
use prelude::contracts::orders_stream_service_client::OrdersStreamServiceClient;
use prelude::contracts::sandbox_service_client::SandboxServiceClient;
use prelude::contracts::stop_orders_service_client::StopOrdersServiceClient;
use std::time::Duration;
use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::service::interceptor::InterceptedService;
use tonic::transport::{Channel, ClientTlsConfig};
use tonic::Request;

use contracts::users_service_client::UsersServiceClient;


pub const PROD_ENDPOINT: &'static str = "https://invest-public-api.tinkoff.ru:443";
pub const SANDBOX_ENDPOINT: &'static str = "https://sandbox-invest-public-api.tinkoff.ru:443";


pub struct ServiceConfig {
    base_url: String,
    auth_token: String,
    user_agent: Option<String>,
    headers: Vec<(&'static str, String)>,
}

impl ServiceConfig {
    fn new<S>(base_url: S, auth_token: S) -> Self where S: Into<String>{
        Self {
            base_url: base_url.into(),
            auth_token: auth_token.into(),            
            user_agent: None,
            headers: vec![],
        }
    }
    fn user_agent<S>(&mut self, user_agent: S) where S: Into<String> {
        self.user_agent= Some(user_agent.into());
    }
    fn headers(&mut self, headers: Vec<(&'static str, String)>) {
        headers.into_iter().map(|x| self.headers.push((x.0, x.1))).count();
    }
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
    pub fn new<S>(base_url: S, auth_token: S) -> GrpcClientFactory<NotReady>
    where S: Into<String> {
        let tls_config = ClientTlsConfig::new().with_native_roots();
        let base_url = base_url.into();
        let auth_token = auth_token.into();
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
    }
}

impl GrpcClientFactory<Ready> {
    pub fn users_service(&self) -> UsersServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
        let channel = self.channel.clone();
         UsersServiceClient::with_interceptor(channel.unwrap(), move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }
    pub fn orders_service(&self) -> OrdersServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
        let channel = self.channel.clone();
         OrdersServiceClient::with_interceptor(channel.unwrap(), move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }
    pub fn orders_stream_service(&self) -> OrdersStreamServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
        let channel = self.channel.clone();
         OrdersStreamServiceClient::with_interceptor(channel.unwrap(), move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }
    pub fn stop_orders_service(&self) -> StopOrdersServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
        let channel = self.channel.clone();
         StopOrdersServiceClient::with_interceptor(channel.unwrap(), move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }
    pub fn operations_service(&self) -> OperationsServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
        let channel = self.channel.clone();
         OperationsServiceClient::with_interceptor(channel.unwrap(), move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }
    pub fn operations_stream_service(&self) -> OperationsStreamServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
        let channel = self.channel.clone();
         OperationsStreamServiceClient::with_interceptor(channel.unwrap(), move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }

    pub fn instruments_service(&self) -> InstrumentsServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
        let channel = self.channel.clone();
         InstrumentsServiceClient::with_interceptor(channel.unwrap(), move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }
    pub fn marketdata_service(&self) -> MarketDataServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
        let channel = self.channel.clone();
         MarketDataServiceClient::with_interceptor(channel.unwrap(), move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }
    pub fn marketdata_stream_service(&self) -> MarketDataStreamServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
        let channel = self.channel.clone();
         MarketDataStreamServiceClient::with_interceptor(channel.unwrap(), move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }
    pub fn sandbox_service(&self) -> SandboxServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
        let channel = self.channel.clone();
         SandboxServiceClient::with_interceptor(channel.unwrap(), move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }


}
