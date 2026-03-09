use crate::infrastructure::nats::jetstream::NatsStreamsRegistry;
use crate::infrastructure::nats::runner::NatsSubscribersRunner;

pub fn create_runner(streams_registry: NatsStreamsRegistry) -> NatsSubscribersRunner {
    let runner = NatsSubscribersRunner::new(streams_registry);
    runner
}
