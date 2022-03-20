#![allow(dead_code)]
#![allow(unused_variables)]
extern crate tokio;
use std::net::TcpListener;

use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct SubscriptionData {
    pub name: String,
    pub email: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<SubscriptionData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().service(health).service(subscribe))
        .listen(listener)?
        .run();

    Ok(server)
}
