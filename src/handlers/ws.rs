use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;

use crate::models::Article;

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<Article>,
}

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    // Task to send broadcast messages to client
    let send_task = tokio::spawn(async move {
        while let Ok(article) = rx.recv().await {
            let msg = serde_json::json!({
                "type": "news",
                "data": article
            });
            if sender
                .send(Message::Text(msg.to_string().into()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    // Task to receive messages from client (for ping/pong or commands)
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => break,
                Message::Ping(data) => {
                    // Pong is handled automatically by axum
                    tracing::debug!("Received ping: {:?}", data);
                }
                Message::Text(text) => {
                    tracing::debug!("Received text: {}", text);
                    // Could handle commands here in the future
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    tracing::info!("WebSocket client disconnected");
}
