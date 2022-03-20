use std::net::TcpListener;

use secrecy::ExposeSecret;
use sent::startup::run;
use sent::telemetry::init_subscriber;
use sent::{configuration::get_configuration, telemetry::get_subscriber};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("sent", "info", std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to port 8000");

    run(listener, connection_pool)?.await
}
