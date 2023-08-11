mod alpaca_api;
mod stream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let api_key = std::env::var("API_KEY")?;
    let api_secret = std::env::var("SECRET")?;

    let mut stream =
        stream::connect_to_server("wss://stream.data.alpaca.markets:443/v2/iex").await?;

    let auth_message = alpaca_api::ActionMessage::new("auth")
        .key(&api_key)
        .secret(&api_secret);
    let suscribe_to_stocks = alpaca_api::ActionMessage::new("subscribe").bars(&vec!["AAL"]);

    stream
        .send_message(serde_json::to_string(&auth_message)?)
        .await?;

    stream
        .send_message(serde_json::to_string(&suscribe_to_stocks)?)
        .await?;

    stream.read_messages().await?;

    Ok(())
}
