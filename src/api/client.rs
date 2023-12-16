use crate::api::transport::Transport;

pub struct CoinGecko {
    transport: Transport,
}

impl CoinGecko {
    pub fn transport(&self) -> &Transport {
        &self.transport
    }
}
