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
use std::str::FromStr;
use std::time::Duration;
use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::service::interceptor::InterceptedService;
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};
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
    pub fn new<S>(base_url: S, auth_token: S) -> Self where S: Into<String>{
        Self {
            base_url: base_url.into(),
            auth_token: auth_token.into(),            
            user_agent: None,
            headers: vec![],
        }
    }
    pub fn user_agent<S>(&mut self, user_agent: S) where S: Into<String> {
        self.user_agent= Some(user_agent.into());
    }
    pub fn headers(&mut self, headers: Vec<(&'static str, String)>) {
        headers.into_iter().map(|x| self.headers.push((x.0, x.1))).count();
    }
}

pub struct ServiceClientFactory {
    base_url: String,
    // tls_config: ClientTlsConfig,
    endpoint: Endpoint,
    user_agent: String,
    metadata: MetadataMap,
}

impl TryFrom<ServiceConfig> for ServiceClientFactory{
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: ServiceConfig) -> Result<Self, Self::Error> {
        let ServiceConfig {
            base_url,
            auth_token,
            user_agent,
            headers, 
        } = value;
        let mut metadata: MetadataMap = MetadataMap::new();
        for header in headers {
            metadata.insert(header.0, header.1.parse()?);
        }
        let user_agent = match user_agent {
            Some(user_agent) => user_agent,
            None => "sillent/invest-api-rust-sdk".to_owned(),
        };
        let token: MetadataValue<_>= format!("Bearer {}", auth_token).parse()?;
        metadata.insert("authorization", token);
        let tls_config = ClientTlsConfig::new().with_native_roots();
        let endpoint = Channel::from_shared(base_url.clone())?
            .tls_config(tls_config)?;
        Ok(ServiceClientFactory {
            base_url,
            // tls_config,
            user_agent,
            endpoint,
            metadata,
        })
    }
}


impl ServiceClientFactory {
    // pub fn new<S>(base_url: S, auth_token: S) -> ServiceClientFactory
    // where S: Into<String> {
    //     let tls_config = ClientTlsConfig::new().with_native_roots();
    //     let base_url = base_url.into();
    //     let auth_token = auth_token.into();
    //     let endpoint = Channel::from_shared(base_url.clone()).unwrap().tls_config(tls_config.clone()).unwrap();
    //     Self {
    //         base_url,
    //         auth_token,
    //         tls_config,
    //         endpoint, 
    //         metadata: MetadataMap::new(),
    //     }
    // }
    pub fn new<S>(base_url: S, auth_token: S) -> ServiceClientFactory where S: Into<String>{
        Self::try_from(ServiceConfig::new(base_url, auth_token)).unwrap()
    }

    // pub async fn connect(
    //     self,
    //     timeout: Duration,
    //     rate_limit: (u64, Duration),
    // ) -> Result<ServiceClientFactory, Box<dyn std::error::Error>> {
    //     let channel = Channel::from_shared(self.base_url.clone())?
    //         .tls_config(self.tls_config.clone())?
    //         .rate_limit(rate_limit.0, rate_limit.1)
    //         .timeout(timeout)
    //         .connect()
    //         .await?;
    //     Ok(ServiceClientFactory {
    //         base_url: self.base_url,
    //         auth_token: self.auth_token,
    //         tls_config: self.tls_config,
    //         metadata: self.metadata,
    //         channel: Some(channel),
    //     })
    // }
}

impl ServiceClientFactory {
    pub async fn users_service2(&self, rate_limit: Option<(u64, Duration)>, timeout: Option<Duration>) -> Result<UsersServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>>, Box<dyn std::error::Error>>
    {
        let mut endpoint = self.endpoint.clone();
        if let Some(timeout) = timeout {
            endpoint = endpoint.timeout(timeout)
        }
        if let Some(rate_limit) = rate_limit {
            endpoint = endpoint.rate_limit(rate_limit.0, rate_limit.1)
        }
        let channel = endpoint.user_agent(&self.user_agent)?.connect().await?;
        let metadata = self.metadata.clone();
        Ok(UsersServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
            metadata.iter().map(|x| {
                match x {
                    tonic::metadata::KeyAndValueRef::Ascii(k, v) => {
                        req.metadata_mut().insert(k, v.clone());
                    },
                    tonic::metadata::KeyAndValueRef::Binary(k, v) => {},
                }
            }).count();

            Ok(req)
        }))
    }
    // pub fn users_service(&self) -> UsersServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
    //     let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
    //     let channel = self.channel.clone().unwrap();
    //     UsersServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
    //         req.metadata_mut().insert("authorization", token.clone());
    //         Ok(req)
    //     })
    // }
    // pub fn orders_service(&self) -> OrdersServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
    //     let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
    //     let channel = self.channel.clone().unwrap();
    //     OrdersServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
    //         req.metadata_mut().insert("authorization", token.clone());
    //         Ok(req)
    //     })
    // }
    // pub fn orders_stream_service(&self) -> OrdersStreamServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
    //     let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
    //     let channel = self.channel.clone().unwrap();
    //     OrdersStreamServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
    //         req.metadata_mut().insert("authorization", token.clone());
    //         Ok(req)
    //     })
    // }
    // pub fn stop_orders_service(&self) -> StopOrdersServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
    //     let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
    //     let channel = self.channel.clone().unwrap();
    //     StopOrdersServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
    //         req.metadata_mut().insert("authorization", token.clone());
    //         Ok(req)
    //     })
    // }
    // pub fn operations_service(&self) -> OperationsServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
    //     let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
    //     let channel = self.channel.clone().unwrap();
    //     OperationsServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
    //         req.metadata_mut().insert("authorization", token.clone());
    //         Ok(req)
    //     })
    // }
    // pub fn operations_stream_service(&self) -> OperationsStreamServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
    //     let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
    //     let channel = self.channel.clone().unwrap();
    //     OperationsStreamServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
    //         req.metadata_mut().insert("authorization", token.clone());
    //         Ok(req)
    //     })
    // }
    // pub fn instruments_service(&self) -> InstrumentsServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
    //     let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
    //     let channel = self.channel.clone().unwrap();
    //     InstrumentsServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
    //         req.metadata_mut().insert("authorization", token.clone());
    //         Ok(req)
    //     })
    // }
    // pub fn marketdata_service(&self) -> MarketDataServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
    //     let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
    //     let channel = self.channel.clone().unwrap();
    //     MarketDataServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
    //         req.metadata_mut().insert("authorization", token.clone());
    //         Ok(req)
    //     })
    // }
    // pub fn marketdata_stream_service(&self) -> MarketDataStreamServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
    //     let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
    //     let channel = self.channel.clone().unwrap();
    //     MarketDataStreamServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
    //         req.metadata_mut().insert("authorization", token.clone());
    //         Ok(req)
    //     })
    // }
    // pub fn sandbox_service(&self) -> SandboxServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
    //     let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
    //     let channel = self.channel.clone().unwrap();
    //     SandboxServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
    //         req.metadata_mut().insert("authorization", token.clone());
    //         Ok(req)
    //     })
    // }
}
