use sent::startup::SentApplication;
use sent::telemetry::init_subscriber;
use sent::{configuration::get_configuration, telemetry::get_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("sent", "info", std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");

    let (_, server) = SentApplication::build(configuration, true).await.unwrap();

    server.await
}
