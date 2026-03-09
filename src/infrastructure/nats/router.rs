use crate::infrastructure::nats::runner::Handler;
use async_nats::jetstream::consumer;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ConsumerOpts {
    pub pull: consumer::pull::Config,
}

#[derive(Clone)]
pub(crate) struct Subscription {
    pub(crate) stream_name: String,
    pub(crate) subject: String,
    pub(crate) handler: Arc<dyn Handler>,
    pub(crate) consumer_opts: Option<ConsumerOpts>,
}

pub struct NatsRouter {
    pub(crate) subscriptions: Vec<Subscription>,
}

impl NatsRouter {
    pub fn new() -> Self {
        Self {
            subscriptions: Vec::new(),
        }
    }

    pub fn subscriber<H: Handler>(
        &mut self,
        stream_name: &str,
        subject: &str,
        handler: H,
    ) -> &mut Self {
        let sub = Subscription {
            stream_name: stream_name.to_string(),
            subject: subject.to_string(),
            handler: Arc::new(handler),
            consumer_opts: None,
        };
        self.subscriptions.push(sub);
        self
    }

    pub fn subscriber_with_opts<H: Handler>(
        &mut self,
        stream_name: &str,
        subject: &str,
        handler: H,
        consumer_opts: ConsumerOpts,
    ) -> &mut Self {
        let sub = Subscription {
            stream_name: stream_name.to_string(),
            subject: subject.to_string(),
            handler: Arc::new(handler),
            consumer_opts: Some(consumer_opts),
        };
        self.subscriptions.push(sub);
        self
    }

    pub fn extend_router(&mut self, router: NatsRouter) -> &mut Self {
        self.subscriptions.extend(router.subscriptions);
        self
    }
}
