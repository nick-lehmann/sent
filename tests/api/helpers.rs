use once_cell::sync::Lazy;
use sent::{
    configuration::get_configuration,
    startup::SentApplication,
    telemetry::{get_subscriber, init_subscriber},
};
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info";
    let subscriber_name = "test";
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub async fn spawn_app() -> SentApplication {
    Lazy::force(&TRACING);

    let configuration = {
        let mut config = get_configuration().expect("Failed to read configuration.");

        config.application.host = "127.0.0.1".to_string();
        config.application.port = 0;
        config.database.database_name = Uuid::new_v4().to_string();

        config
    };

    let (app, server) = SentApplication::build(configuration, false).await.unwrap();

    let _ = tokio::spawn(server);

    app
}
