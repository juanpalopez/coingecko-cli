use crate::api::transport::Transport;

pub struct CoinGecko {
    transport: Transport,
}

impl CoinGecko {
    pub fn new(transport: Transport) -> Self {
        CoinGecko { transport }
    }
    pub fn transport(&self) -> &Transport {
        &self.transport
    }
}
