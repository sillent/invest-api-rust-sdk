use std::time::Duration;
use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::service::interceptor::InterceptedService;
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};
use tonic::{Request, Status};

use crate::investment_api::users_service_client::UsersServiceClient;

pub struct GrpcClientFactory {
    base_url: String,
    auth_token: String,
    tls_config: ClientTlsConfig,
    channel: Option<Channel>,
    metadata: MetadataMap,
}

impl GrpcClientFactory {
    pub fn new(base_url: String, auth_token: String) -> Self {
        let tls_config = ClientTlsConfig::new().with_native_roots();
        Self {
            base_url,
            auth_token,
            tls_config,
            channel: None,
            metadata: MetadataMap::new(),
        }
    }

    pub async fn connect(
        &mut self,
        timeout: Duration,
        rate_limit: (u64, Duration),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse()?;
        let channel = Channel::from_shared(self.base_url.clone())?
            .tls_config(self.tls_config.clone())?
            .rate_limit(rate_limit.0, rate_limit.1)
            .timeout(timeout)
            .connect()
            .await?;
        self.channel = Some(channel);
        Ok(())
    }


    pub async fn user_service(&self, channel: Channel) -> UsersServiceClient<InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, tonic::Status>>> {
        let token: MetadataValue<_>= format!("Bearer {}", self.auth_token).parse().expect("parsing correct");
         UsersServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        })
    }

}
