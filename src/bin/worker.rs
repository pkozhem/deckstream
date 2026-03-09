use deckstream::application::bootstrap;
use deckstream::application::settings::Settings;
use deckstream::infrastructure;
use deckstream::infrastructure::os::control::Control;
use tokio::sync::watch;
use tracing_subscriber;

#[tokio::main]
async fn main() -> infrastructure::nats::r#type::Result<()> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let settings = Settings::load();

    let nats_url = format!(
        "nats://{}:{}",
        settings.nats_host, settings.nats_client_port,
    );
    let nats_client = bootstrap::jetstream::create_nats_client(nats_url).await?;
    let js = bootstrap::jetstream::create_jetstream_client(nats_client);

    let mut streams_registry = bootstrap::stream::create_streams_registry(js);
    bootstrap::stream::register_streams(&mut streams_registry, settings).await?;

    let mut runner = bootstrap::runner::create_runner(streams_registry);
    let routers = bootstrap::router::get_registered_routers();

    for router in routers {
        runner.include_router(router);
    }

    let (shutdown_tx, shutdown_rx) = watch::channel(Control::Run);
    let runner_handle = tokio::spawn(async move { runner.run(shutdown_rx).await });

    infrastructure::os::shutdown::wait_shutdown_signal().await?;
    tracing::info!("Shutting down...");
    let _ = shutdown_tx.send(Control::Shutdown);
    runner_handle.await??;

    Ok(())
}
