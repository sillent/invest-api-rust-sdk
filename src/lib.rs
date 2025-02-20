pub(crate) mod investment_api {
    tonic::include_proto!("tinkoff.public.invest.api.contract.v1");
}

pub use investment_api::*;
