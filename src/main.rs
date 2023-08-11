mod stream;

#[tokio::main]
async fn main()-> anyhow::Result<()> {
    stream::connect_to_server("wss://stream.data.alpaca.markets:443/v2/iex").await?;

    Ok(())
}
