# RustAPI WebSocket

**Real-time bidirectional communication made simple.**

Built on `tokio-tungstenite`, this crate provides a first-class WebSocket extractor for RustAPI.

## Usage

```rust
use rustapi_ws::{WebSocket, Message};

#[get("/chat")]
async fn chat_handler(ws: WebSocket) -> impl Responder {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            println!("Received: {}", text);
            socket.send(Message::Text("Echo!".into())).await.unwrap();
        }
    }
}
```

## Features
- **Auto-Upgrade**: Handles the HTTP 101 Switching Protocols handshake.
- **Channels**: Built-in pub/sub for broadcast scenarios (chat rooms).
- **Ping/Pong**: Automatic heartbeat management.
