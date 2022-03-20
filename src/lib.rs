extern crate tokio;
use std::net::TcpListener;

use actix_web::{dev::Server, get, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().service(health))
        .listen(listener)?
        .run();

    Ok(server)
}
