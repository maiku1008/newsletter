use newsletter::telemetry::{get_subscriber, init_subscriber};
use newsletter::{configuration, startup};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = configuration::get_configuration().expect("Failed to get configuration");
    let connection = PgPool::connect(&config.database.conn_string().expose_secret())
        .await
        .expect("failed to connect to postgres.");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    startup::run(listener, connection)?.await
}
