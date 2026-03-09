use crate::domain::orders::service::orders_service;
use crate::domain::orders::r#type::OrderCreated;
use crate::domain::requests::service::requests_service;
use crate::domain::requests::r#type::RequestCreated;
use crate::infrastructure::nats::error::BoxError;
use async_nats::jetstream::Message as JetStreamMessage;

pub async fn handle_orders(msg: JetStreamMessage) -> std::result::Result<(), BoxError> {
    let payload = String::from_utf8(msg.payload.to_vec()).map_err(|e| Box::new(e) as BoxError)?;
    let event = OrderCreated { payload };
    orders_service(event).map_err(|e| Box::new(e) as BoxError)?;
    msg.ack().await?;
    Ok(())
}

pub async fn handle_requests(msg: JetStreamMessage) -> std::result::Result<(), BoxError> {
    let payload = String::from_utf8(msg.payload.to_vec()).map_err(|e| Box::new(e) as BoxError)?;
    let event = RequestCreated { payload };
    requests_service(event).map_err(|e| Box::new(e) as BoxError)?;
    msg.ack().await?;
    Ok(())
}
