use crate::domain::kernel::result::Result;
use crate::domain::requests::r#type::RequestCreated;

pub fn requests_service(event: RequestCreated) -> Result<()> {
    tracing::info!("request created: {:?}", event.payload.trim());
    Ok(())
}
