use crate::{Error, executor};
use aum_core::prelude::{Request, Response};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use tracing::error;
pub struct Server;

impl Server {
    pub async fn new<S, Wm, M>(
        bind: &str,
        executor: Arc<crate::executor::Executor<S, Wm, M>>,
    ) -> Result<Self, Error>
    where
        S: aum_core::prelude::Storage + Send + 'static + Sync,
        Wm: aum_core::prelude::WalletManager + Send + 'static + Sync,
        M: aum_core::prelude::Monitor<WalletManager = Wm> + Send + 'static + Sync,
    {
        let listener = TcpListener::bind(bind).await?;

        while let Ok((stream, _)) = listener.accept().await {
            let executor = Arc::clone(&executor);
            tokio::spawn(async move {
                let ws_stream = match accept_async(stream).await {
                    Ok(ws_stream) => ws_stream,
                    Err(e) => {
                        eprintln!("Error during WebSocket handshake: {}", e);
                        return;
                    }
                };
                let (mut write, mut read) = ws_stream.split();
                while let Some(Ok(msg)) = read.next().await {
                    match msg {
                        Message::Text(text) => {
                            let string = text.to_string();
                            let req: Request = match serde_json::from_str(&string) {
                                Ok(req) => req,
                                Err(_) => {
                                    if let Err(e) =
                                        write.send(error_into_message(Error::WrongRequest)).await
                                    {
                                        error_websocket(e);
                                    }
                                    continue;
                                }
                            };
                            let response = match executor.execute(req).await {
                                Ok(response) => response_into_message(response),
                                Err(e) => error_into_message(e),
                            };
                        }
                        Message::Ping(_) => {
                            if let Err(e) = write.send(Message::Pong((&[] as &[u8]).into())).await {
                                error_websocket(e);
                            }
                            continue;
                        }
                        _ => continue,
                    }
                }
            });
        }
        Ok(Self {})
    }
}

fn error_into_message(e: Error) -> Message {
    Message::Text(e.to_string().into())
}
fn response_into_message(res: Response) -> Message {
    Message::Text(res.to_string().into())
}
fn error_websocket(e: tokio_tungstenite::tungstenite::Error) {
    error!("WebSocket error: {}", e);
}
