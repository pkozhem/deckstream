use crate::infrastructure::nats::router::{ConsumerOpts, NatsRouter};
use crate::presentation::broker::v1::endpoints::events::{self, handle_orders};
use async_nats::jetstream::consumer;

pub fn get_events_router() -> NatsRouter {
    let mut router = NatsRouter::new();
    router.subscriber_with_opts(
        "events",
        "orders",
        handle_orders,
        ConsumerOpts {
            pull: consumer::pull::Config {
                description: Some("orders subscriber".to_string()),
                filter_subject: "orders".to_string(),
                ..Default::default()
            },
        },
    );
    router.subscriber("events", "requests", events::handle_requests);
    return router;
}
