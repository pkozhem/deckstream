use async_nats::jetstream::Context;

pub struct NatsPublisher {
    js: Context,
}

impl NatsPublisher {
    pub fn new(js: Context) -> Self {
        Self { js }
    }

    pub async fn publish(
        &self,
        subject: String,
        payload: bytes::Bytes,
    ) -> Result<(), async_nats::Error> {
        self.js.publish(subject, payload).await?;
        Ok(())
    }
}
