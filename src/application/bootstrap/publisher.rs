use crate::infrastructure::nats::publisher::NatsPublisher;
use async_nats::jetstream::Context;

pub fn create_publisher(js: Context) -> NatsPublisher {
    let publisher = NatsPublisher::new(js);
    publisher
}
