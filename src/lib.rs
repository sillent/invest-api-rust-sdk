pub mod other;
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
use std::time::Duration;
use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::service::interceptor::InterceptedService;
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};
use tonic::Request;

use contracts::users_service_client::UsersServiceClient;

pub const PROD_ENDPOINT: &'static str = "https://invest-public-api.tinkoff.ru:443";
pub const SANDBOX_ENDPOINT: &'static str = "https://sandbox-invest-public-api.tinkoff.ru:443";
const DEFAULT_USER_AGENT: &'static str = "sillent/invest-api-rust-sdk";

pub struct ServiceFactoryBuilder {
    base_url: Option<String>,
    token: Option<String>,
    user_agent: Option<String>,
    headers: Vec<(&'static str, String)>,
    rate_limit: Option<(u64, Duration)>,
    timeout: Option<Duration>,
    tcp_keepalive: Option<Duration>,
}

impl ServiceFactoryBuilder {
    pub fn new() -> Self {
        Self {
            base_url: None,
            token: None,
            user_agent: None,
            headers: vec![],
            rate_limit: None,
            timeout: None,
            tcp_keepalive: None,
        }
    }

    pub fn base_url<S: Into<String>>(self, base_url: S) -> Self {
        Self {
            base_url: Some(base_url.into()),
            ..self
        }
    }

    pub fn token<S: Into<String>>(self, token: S) -> Self {
        Self {
            token: Some(token.into()),
            ..self
        }
    }
    pub fn user_agent<S: Into<String>>(self, user_agent: S) -> Self {
        Self {
            user_agent: Some(user_agent.into()),
            ..self
        }
    }
    pub fn headers(self, headers: Vec<(&'static str, String)>) -> Self {
        Self { headers, ..self }
    }

    pub fn rate_limit(self, rate_limit: (u64, Duration)) -> Self {
        Self {
            rate_limit: Some(rate_limit),
            ..self
        }
    }
    pub fn timeout(self, timeout: Duration) -> Self {
        Self {
            timeout: Some(timeout),
            ..self
        }
    }
    pub fn tcp_keepalive(self, tcp_keepalive: Duration) -> Self {
        Self {
            tcp_keepalive: Some(tcp_keepalive),
            ..self
        }
    }
    pub fn build(self) -> Result<ServiceFactory, Box<dyn std::error::Error>> {
        let ServiceFactoryBuilder {
            base_url,
            token,
            user_agent,
            headers,
            rate_limit,
            timeout,
            tcp_keepalive,
        } = self;
        let base_url = match base_url {
            Some(base_url) => base_url,
            None => PROD_ENDPOINT.to_owned(),
        };
        let tls_config = ClientTlsConfig::new().with_native_roots();
        let mut metadata: MetadataMap = MetadataMap::new();
        for header in headers {
            metadata.insert(header.0, header.1.parse()?);
        }
        if let Some(token) = token {
            let token: MetadataValue<_> = format!("Bearer {}", token).parse()?;
            metadata.insert("authorization", token);
        }
        let user_agent = match user_agent {
            Some(user_agent) => user_agent,
            None => DEFAULT_USER_AGENT.to_owned(),
        };
        let mut endpoint = Endpoint::from_shared(base_url)?
            .tls_config(tls_config)?
            .user_agent(user_agent)?
            .tcp_keepalive(tcp_keepalive);
        if let Some(rate_limit) = rate_limit {
            endpoint = endpoint.rate_limit(rate_limit.0, rate_limit.1);
        }
        if let Some(timeout) = timeout {
            endpoint = endpoint.timeout(timeout);
        }
        Ok(ServiceFactory { endpoint, metadata })
    }
}

pub struct ServiceFactory {
    endpoint: Endpoint,
    metadata: MetadataMap,
}

impl ServiceFactory {
    pub fn builder() -> ServiceFactoryBuilder {
        ServiceFactoryBuilder::new()
    }

    pub fn users_service(
        &self,
        rate_limit: Option<(u64, Duration)>,
        timeout: Option<Duration>,
    ) -> UsersServiceClient<
        InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>,
    > {
        let mut endpoint = self.endpoint.clone();
        if let Some(timeout) = timeout {
            endpoint = endpoint.timeout(timeout)
        }
        if let Some(rate_limit) = rate_limit {
            endpoint = endpoint.rate_limit(rate_limit.0, rate_limit.1)
        }
        let channel = endpoint.connect_lazy();
        let metadata = self.metadata.clone();
        UsersServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
            metadata
                .iter()
                .map(|x| match x {
                    tonic::metadata::KeyAndValueRef::Ascii(k, v) => {
                        req.metadata_mut().insert(k, v.clone());
                    }
                    tonic::metadata::KeyAndValueRef::Binary(k, v) => {
                        req.metadata_mut().insert_bin(k, v.clone());
                    }
                })
                .count();
            println!("req meta = {:?}", req.metadata());

            Ok(req)
        })
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
