use tokio::signal::unix::{SignalKind, signal};

#[cfg(unix)]
pub async fn wait_shutdown_signal() -> std::io::Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = sigint.recv() => {}
        _ = sigterm.recv() => {}
    }

    Ok(())
}
