use crate::infrastructure::nats::router::NatsRouter;
use crate::presentation::broker::v1::endpoints::router;

pub fn get_registered_routers() -> Vec<NatsRouter> {
    let events_router = router::get_events_router();
    let routers: Vec<NatsRouter> = vec![events_router];
    routers
}
