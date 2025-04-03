use aum_core::prelude::{Request, Response};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use tungstenite::http::{Method, Request as WsRequest};

pub struct AumAPI {
    bind: String,
}
impl AumAPI {
    pub fn new(bind: &str) -> Self {
        Self {
            bind: bind.to_owned(),
        }
    }
    pub async fn connect(&self) -> Result<AumConnection, Box<dyn std::error::Error>> {
        let url = format!("ws://{}", self.bind);
        let (ws_stream, _) = connect_async(url).await?;

        Ok(AumConnection { ws_stream })
    }
}

pub struct AumConnection {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}
impl AumConnection {
    pub async fn send_request(
        &mut self,
        request: Request,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let request_json = serde_json::to_string(&request)?;
        self.ws_stream
            .send(tungstenite::Message::Text(request_json.into()))
            .await?;

        if let Some(msg) = self.ws_stream.next().await {
            match msg? {
                tungstenite::Message::Text(response_json) => {
                    let response: Response = serde_json::from_str(&response_json)?;
                    Ok(response)
                }
                _ => Err("Unexpected WebSocket message type".into()),
            }
        } else {
            Err("WebSocket stream closed".into())
        }
    }

    pub async fn retrieve_address(&mut self) -> Result<Response, Box<dyn std::error::Error>> {
        self.send_request(Request::RetrieveAddress).await
    }

    pub async fn send_transaction(
        &mut self,
        to: String,
        amount: u64,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        self.send_request(Request::SendTransaction { to, amount })
            .await
    }

    pub async fn send_transaction_from(
        &mut self,
        from: String,
        to: String,
        amount: u64,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        self.send_request(Request::SendTransactionFrom { from, to, amount })
            .await
    }

    pub async fn retrieve_balance(
        &mut self,
        address: String,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        self.send_request(Request::RetrieveBalance { address })
            .await
    }

    pub async fn retrieve_balances(&mut self) -> Result<Response, Box<dyn std::error::Error>> {
        self.send_request(Request::RetrieveBalances).await
    }

    pub async fn list_wallets(&mut self) -> Result<Response, Box<dyn std::error::Error>> {
        self.send_request(Request::ListWallets).await
    }

    pub async fn sync(&mut self) -> Result<Response, Box<dyn std::error::Error>> {
        self.send_request(Request::Sync).await
    }
}
