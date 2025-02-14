use actix_web::{web, App, HttpServer, Responder};


async fn hello() -> impl Responder {
    "Hello, world!"
}