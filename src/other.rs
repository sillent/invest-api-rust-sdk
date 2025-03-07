pub struct ExchangeEndpoint {
    url: String,
    token: String,
}

impl ExchangeEndpoint {
    fn new(url: String, token: String) -> Self {
        Self { url, token }
    }
    async fn channel(&self) -> tonic::transport::Channel {
        
        unimplemented!();
    }

    // fn users_service(channel)
}
