use async_nats::jetstream::Context;
use async_nats::{Client, ConnectError};

pub async fn create_nats_client(nats_url: String) -> std::result::Result<Client, ConnectError> {
    let client = async_nats::connect(nats_url).await?;
    Ok(client)
}

pub fn create_jetstream_client(client: Client) -> Context {
    async_nats::jetstream::new(client)
}
