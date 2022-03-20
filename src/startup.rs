use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes::{health_check, subscribe};

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

pub fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<Server> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(subscribe)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}