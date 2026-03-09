use crate::domain::kernel::result::Result;
use crate::domain::orders::r#type::OrderCreated;

pub fn orders_service(event: OrderCreated) -> Result<()> {
    tracing::info!("order created: {:?}", event.payload.trim());
    Ok(())
}
