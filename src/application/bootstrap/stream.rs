use crate::application::settings;
use crate::infrastructure::nats::jetstream::NatsStreamsRegistry;
use crate::infrastructure::nats::r#type::Result;

pub fn create_streams_registry(js: async_nats::jetstream::Context) -> NatsStreamsRegistry {
    let streams_registry = NatsStreamsRegistry::new(js);
    streams_registry
}

pub async fn register_streams(
    streams_registry: &mut NatsStreamsRegistry,
    settings: settings::Settings,
) -> Result<()> {
    let stream_config = async_nats::jetstream::stream::Config {
        name: settings.events_stream,
        subjects: vec![settings.orders_subject, settings.requests_subject],
        max_messages: 10_000,
        first_sequence: Some(1),
        ..Default::default()
    };
    streams_registry.add_stream(stream_config).await?;
    Ok(())
}
