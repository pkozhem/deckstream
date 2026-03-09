use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub nats_host: String,
    pub nats_client_port: String,
    pub nats_monitor_port: String,

    pub events_stream: String,
    pub orders_subject: String,
    pub requests_subject: String,
}

impl Settings {
    pub fn load() -> Self {
        envy::from_env::<Self>().unwrap()
    }
}
