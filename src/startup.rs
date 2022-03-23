use secrecy::ExposeSecret;
use std::{fmt::format, net::TcpListener};
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::Configuration,
    domain::SubscriberEmail,
    email_client::MockEmailClient,
    routes::{health_check, subscribe},
};

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{Connection, Executor, PgConnection, PgPool};

pub struct SentApplication {
    pub address: String,
    configuration: Configuration,
}

impl SentApplication {
    pub async fn build(
        configuration: Configuration,
        database_present: bool,
    ) -> Result<(Self, Server), std::io::Error> {
        let original_address = &configuration.application.address();
        let listener = TcpListener::bind(&original_address)
            .expect(&format!("Failed to bind to address {}", &original_address));

        let port = listener
            .local_addr()
            .expect("No local address for spawned app")
            .port();

        let address = format!("{}:{}", configuration.application.host, port);

        let app = Self {
            address,
            configuration: configuration,
        };

        if !database_present {
            app.create_database().await;
        }
        let web_db_pool = web::Data::new(app.get_pool().await);

        let email_client = MockEmailClient {
            sender: SubscriberEmail::parse("nick@lehmann.sh".to_string()).unwrap(),
        };

        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .service(health_check)
                .service(subscribe)
                .app_data(web_db_pool.clone())
                .app_data(email_client.clone())
        })
        .listen(listener)?
        .run();

        Ok((app, server))
    }

    pub async fn get_pool(&self) -> PgPool {
        PgPool::connect(
            &self
                .configuration
                .database
                .connection_string()
                .expose_secret(),
        )
        .await
        .expect("Failed to connect to Postgres.")
    }

    pub async fn create_database(&self) {
        let mut connection = PgConnection::connect(
            &self
                .configuration
                .database
                .connection_string_without_db()
                .expose_secret(),
        )
        .await
        .expect("Failed to connect to Postgres");

        connection
            .execute(
                format!(
                    r#"CREATE DATABASE "{}";"#,
                    &self.configuration.database.database_name
                )
                .as_str(),
            )
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(database_error) => {
                    if database_error.code().unwrap() != "42P04" {
                        panic!("Failed to create a database and there is none present to use")
                    }
                }
                _ => panic!("Failed to create a database and there is none present to use"),
            })
            .unwrap();

        let pool = self.get_pool().await;
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to migrate the database");
    }
}
