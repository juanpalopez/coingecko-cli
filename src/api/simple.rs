use crate::api::client::CoinGecko;
use crate::api::error::Error;
use crate::api::response::Response;
use crate::api::transport::Transport;
use crate::api::Method;

enum SimpleSupportedVsCurrenciesParts {
    None,
}

impl SimpleSupportedVsCurrenciesParts {
    fn url(self) -> &'static str {
        match self {
            SimpleSupportedVsCurrenciesParts::None => "/simple/supported_vs_currencies",
        }
    }
}

struct SimpleSupportedVsCurrencies<'a> {
    transport: &'a Transport,
    parts: SimpleSupportedVsCurrenciesParts,
}

impl SimpleSupportedVsCurrencies {
    fn new(transport: &Transport, parts: SimpleSupportedVsCurrenciesParts) -> Self {
        SimpleSupportedVsCurrencies { transport, parts }
    }

    async fn send(&self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let response = self.transport.send(method, &path).await?;
        Ok(response)
    }
}

struct Simple<'a> {
    transport: &'a Transport,
}

impl Simple {
    fn new(transport: &Transport) -> Self {
        Self { transport }
    }

    fn transport(&self) -> &Transport {
        self.transport
    }

    fn vs_supported_currencies(
        &self,
        parts: SimpleSupportedVsCurrenciesParts,
    ) -> SimpleSupportedVsCurrenciesParts {
        SimpleSupportedVsCurrenciesParts::new(self.transport(), parts)
    }
}

impl CoinGecko {
    fn simple(&self) -> Simple {
        Simple::new(self.transport())
    }
}
