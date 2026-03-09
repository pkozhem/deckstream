use deckstream::application::bootstrap;
use deckstream::application::settings::Settings;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    dotenvy::dotenv().ok();
    let settings = Settings::load();

    let nats_url = format!(
        "nats://{}:{}",
        settings.nats_host, settings.nats_client_port,
    );
    let nats_client = bootstrap::jetstream::create_nats_client(nats_url).await?;
    let js = bootstrap::jetstream::create_jetstream_client(nats_client);

    let subjects = [
        settings.orders_subject.clone(),
        settings.requests_subject.clone(),
    ];

    let mut streams_registry = bootstrap::stream::create_streams_registry(js.clone());
    bootstrap::stream::register_streams(&mut streams_registry, settings).await?;

    let mut input = String::new();
    let mut buf = String::new();

    'repl: loop {
        print!("\n[pub | exit] > ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "pub" => {
                print!("subject [{}] > ", subjects.join(" | "));
                io::stdout().flush().unwrap();
                buf.clear();
                io::stdin().read_line(&mut buf).unwrap();
                let subject = buf.trim().to_string();

                if !subjects.contains(&subject) {
                    eprintln!("unknown subject, available: {}", subjects.join(", "));
                    continue 'repl;
                }

                print!("message > ");
                io::stdout().flush().unwrap();
                buf.clear();
                io::stdin().read_line(&mut buf).unwrap();
                let payload = buf.trim().to_string();

                let publisher = bootstrap::publisher::create_publisher(js.clone());
                publisher
                    .publish(subject.to_string(), payload.into())
                    .await?;
                println!("sent -> [{subject}]");
            }
            "exit" | "" => break,
            other => eprintln!("unknown command: {other:?}"),
        }
    }
    Ok(())
}
