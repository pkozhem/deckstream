use crate::infrastructure::nats::r#type::Result;
use async_nats::jetstream::{self, Context, stream};
use std::collections::HashMap;

pub struct NatsStreamsRegistry {
    js: Context,
    streams: HashMap<String, stream::Stream>,
}

impl Clone for NatsStreamsRegistry {
    fn clone(&self) -> Self {
        Self {
            js: self.js.clone(),
            streams: self.streams.clone(),
        }
    }
}

impl NatsStreamsRegistry {
    pub fn new(js: Context) -> Self {
        Self {
            js: js,
            streams: HashMap::new(),
        }
    }

    pub async fn add_stream(&mut self, config: jetstream::stream::Config) -> Result<()> {
        let name = config.name.clone();
        let s = self.js.get_or_create_stream(config).await?;
        self.streams.insert(name, s);
        Ok(())
    }

    pub async fn get_or_load_stream(&mut self, stream_name: &str) -> Result<stream::Stream> {
        if let Some(s) = self.streams.get(stream_name) {
            return Ok(s.clone());
        }
        let s = self.js.get_stream(stream_name).await?;
        self.streams.insert(stream_name.to_string(), s.clone());
        Ok(s)
    }
}
