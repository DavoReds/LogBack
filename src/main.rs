use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, web};
use anyhow::Context;
use env_logger::Env;

async fn ping() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%r %s %b %D"))
            .route("/ping", web::get().to(ping))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .context("Error con el servidor")
}
