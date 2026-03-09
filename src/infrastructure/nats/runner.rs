use crate::infrastructure::nats::error::BoxError;
use crate::infrastructure::nats::jetstream::NatsStreamsRegistry;
use crate::infrastructure::nats::router::{ConsumerOpts, NatsRouter, Subscription};
use crate::infrastructure::nats::r#type::Result;
use crate::infrastructure::os::control::Control;
use async_nats::jetstream::consumer;
use async_nats::jetstream::{Message as JetStreamMessage, stream};
use async_trait::async_trait;
use futures::TryStreamExt;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::sync::watch::error::RecvError;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn handle(&self, msg: JetStreamMessage) -> std::result::Result<(), BoxError>;
}

#[async_trait]
impl<F, Fut> Handler for F
where
    F: Send + Sync + 'static + Fn(JetStreamMessage) -> Fut,
    Fut: std::future::Future<Output = std::result::Result<(), BoxError>> + Send,
{
    async fn handle(&self, msg: JetStreamMessage) -> std::result::Result<(), BoxError> {
        (self)(msg).await
    }
}

pub struct NatsSubscribersRunner {
    streams: NatsStreamsRegistry,
    subscriptions: Vec<Subscription>,
}

impl NatsSubscribersRunner {
    pub fn new(streams: NatsStreamsRegistry) -> Self {
        Self {
            streams,
            subscriptions: Vec::new(),
        }
    }

    pub fn include_router(&mut self, router: NatsRouter) -> &mut Self {
        self.subscriptions.extend(router.subscriptions);
        self
    }

    pub async fn run(mut self, shutdown: watch::Receiver<Control>) -> Result<()> {
        if self.subscriptions.is_empty() {
            tracing::warn!("No subscribers found - nothing to run.");
            return Ok(());
        }

        let mut handles = Vec::with_capacity(self.subscriptions.len());

        for Subscription {
            stream_name,
            subject,
            handler,
            consumer_opts,
        } in self.subscriptions
        {
            let stream = self.streams.get_or_load_stream(&stream_name).await?;
            handles.push(tokio::spawn(Self::run_subscription(
                stream,
                stream_name,
                subject,
                handler,
                consumer_opts,
                shutdown.clone(),
            )));
        }

        Self::await_handles(handles).await
    }

    async fn run_subscription(
        stream: stream::Stream,
        stream_name: String,
        subject: String,
        handler: Arc<dyn Handler>,
        consumer_opts: Option<ConsumerOpts>,
        mut shutdown: watch::Receiver<Control>,
    ) -> Result<()> {
        let name = format!(
            "consumer_{}_{}",
            stream_name.replace('.', "_"),
            subject.replace('.', "_")
        );
        let consumer_config = Self::consumer_config(name, consumer_opts);

        let durable_name = consumer_config
            .durable_name
            .clone()
            .expect("durable_name is always set");
        let consumer = stream
            .get_or_create_consumer(&durable_name, consumer_config)
            .await?;

        tracing::info!(
            "Pull consumer started (stream='{}', subject='{}')",
            stream_name,
            subject
        );

        let mut messages = consumer.messages().await?;

        loop {
            tokio::select! {
                maybe_msg = messages.try_next() => {
                    match maybe_msg {
                        Ok(Some(msg)) => Self::handle_message(&subject, handler.clone(), msg).await,
                        Ok(None) => if Self::handle_stream_end(&stream_name, &subject) { break },
                        Err(e) => Self::handle_receive_error(&stream_name, &subject, e),
                    }
                }
                changed = shutdown.changed() => {
                    if Self::should_stop(&mut shutdown, changed, &stream_name, &subject) {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn consumer_config(
        durable_name: String,
        consumer_opts: Option<ConsumerOpts>,
    ) -> consumer::pull::Config {
        match consumer_opts {
            Some(mut opts) => {
                opts.pull.durable_name = Some(durable_name);
                opts.pull
            }
            None => consumer::pull::Config {
                durable_name: Some(durable_name),
                ..Default::default()
            },
        }
    }

    async fn handle_message(subject: &str, handler: Arc<dyn Handler>, msg: JetStreamMessage) {
        if msg.subject.as_str() != subject {
            if let Err(e) = msg.ack().await {
                tracing::error!("Failed to ack skipped message: {}", e);
            }
            return;
        }

        let subject_for_handler = subject.to_string();
        tokio::spawn(async move {
            if let Err(e) = handler.handle(msg.clone()).await {
                tracing::error!("Handler error for {}: {}", subject_for_handler, e);
            }
        });
    }

    fn handle_stream_end(stream_name: &str, subject: &str) -> bool {
        tracing::warn!(
            "Message stream ended (stream='{}', subject='{}')",
            stream_name,
            subject
        );
        true
    }

    fn handle_receive_error<E: std::fmt::Display>(stream_name: &str, subject: &str, e: E) {
        tracing::error!(
            "Error receiving message (stream='{}', subject='{}'): {}",
            stream_name,
            subject,
            e
        );
    }

    fn should_stop(
        shutdown: &mut watch::Receiver<Control>,
        changed: std::result::Result<(), RecvError>,
        stream_name: &str,
        subject: &str,
    ) -> bool {
        if changed.is_err() {
            tracing::info!(
                "Shutdown channel was closed unexpectedly (stream='{}', subject='{}')",
                stream_name,
                subject
            );
            return true;
        }

        match shutdown.borrow_and_update().clone() {
            Control::Shutdown => {
                tracing::info!(
                    "Shutting down consumer (stream='{}', subject='{}')",
                    stream_name,
                    subject
                );
                true
            }
            Control::Run => false,
        }
    }

    async fn await_handles(handles: Vec<tokio::task::JoinHandle<Result<()>>>) -> Result<()> {
        for handle in handles {
            handle.await??;
        }
        Ok(())
    }
}
