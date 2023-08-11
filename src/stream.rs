use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tungstenite::protocol::Message;

pub async fn connect_to_server(hostname: &str) -> Result<()> {
    let (stream, _) = tokio_tungstenite::connect_async(hostname).await?;

    println!("Successfully connected to websocket server");

    let (mut write, mut read) = stream.split();

    let auth_message = json!({"action": "auth", "key": "blah", "secret": "test"});
    write.send(Message::Text(auth_message.to_string())).await?;

    loop {
        let Some(message) = read.next().await else { continue };
        match message {
            Ok(msg) => match msg {
                Message::Text(txt) => {
                    println!("Received a message: {:#?}", txt);
                }
                _ => println!("Not text"),
            },
            Err(e) => {
                eprintln!("Error while receiving a message: {:?}", e);
            }
        }
    }

    Ok(())
}
