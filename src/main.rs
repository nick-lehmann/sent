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
    let address = configuration.application.address();
    let listener =
        TcpListener::bind(&address).expect(&format!("Failed to bind to address {}", &address));

    run(listener, connection_pool)?.await
}
