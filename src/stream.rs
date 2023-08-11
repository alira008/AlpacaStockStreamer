use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncReadExt;
use tungstenite::protocol::Message;

pub async fn connect_to_server(hostname: &str) -> Result<Session> {
    let (stream, _) = tokio_tungstenite::connect_async(hostname).await?;

    println!("Successfully connected to websocket server");

    Ok(Session::new(stream))
}

pub struct Session {
    ws: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
}

impl Session {
    fn new(
        stream: tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    ) -> Self {
        Self { ws: stream }
    }

    pub async fn send_message(&mut self, msg: String) -> Result<()> {
        println!("Sending msg: {}", &msg);
        let message = Message::Text(msg);
        self.ws.send(message).await?;
        self.ws.flush().await?;

        Ok(())
    }

    pub async fn read_single_message(&mut self) -> Result<()> {
        if let Some(result_msg) = self.ws.next().await {
            let msg = result_msg?;

            match msg {
                Message::Text(text_message) => println!("Text: {}", text_message),
                Message::Binary(binary_message) => {
                    println!("Binary Message: {:#?}", binary_message)
                }
                Message::Ping(_) => println!("Ping received"),
                Message::Pong(_) => println!("Pong received"),
                Message::Close(_) => println!("Close message received"),
                Message::Frame(_) => println!("Received raw frame"),
            }
        }

        Ok(())
    }

    fn header_length(bytes: &[u8]) -> Option<usize> {
        if bytes.len() < 2 {
            return None; // Not enough data to determine.
        }

        // Extract the mask bit and payload length from the second byte.
        let mask_bit = (bytes[1] & 0b1000_0000) != 0;
        let payload_len = bytes[1] & 0b0111_1111;

        let header_size = match payload_len {
            0..=125 => 2, // No extended payload length.
            126 => 4,     // 2-byte extended payload length.
            127 => 10,    // 8-byte extended payload length.
            _ => return None,
        };

        if mask_bit {
            Some(header_size + 4) // Add 4 bytes for the masking key.
        } else {
            Some(header_size)
        }
    }

    pub async fn read_messages(&mut self) -> Result<()> {
        while let Some(result_msg) = self.ws.next().await {
            if result_msg.is_err() {
                continue;
            }

            let msg = result_msg.unwrap();

            match msg {
                Message::Text(text_message) => println!("Text: {}", text_message),
                Message::Binary(binary_message) => {
                    println!("Binary Message: {:#?}", binary_message)
                }
                Message::Ping(_) => println!("Ping received"),
                Message::Pong(_) => println!("Pong received"),
                Message::Close(_) => println!("Close message received"),
                Message::Frame(_) => println!("Received raw frame"),
            }
        }

        Ok(())
    }

    pub async fn read_messages_raw(&mut self) -> Result<()> {
        let mut buf: [u8; 80_000] = [0; 80_000];
        loop {
            let bytes_read = self.ws.get_mut().read(&mut buf).await?;
            let header_size = Session::header_length(&buf[..bytes_read]);

            if header_size.is_none() {
                println!("Socket closed.");
                break;
            }

            println!("Bytes read: {}", bytes_read);
            // First two bytes are reserved for the header
            // After first two bytes, we have message
            let msg_slice = std::str::from_utf8(&buf[header_size.unwrap()..bytes_read])?;
            println!("{}", msg_slice);
        }

        Ok(())
    }
}
