use actix_web::{
    App, HttpServer,
    dev::Server,
    middleware::{Compress, Logger},
};
use std::net::TcpListener;

use crate::{config::Settings, rutas::configurar_rutas};

pub fn build_server(cfg: &Settings) -> Result<Server, anyhow::Error> {
    let address = format!("{}:{}", cfg.server.host, cfg.server.port);
    let listener = TcpListener::bind(&address)?;

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%r %s %b %D"))
            .wrap(Compress::default())
            .configure(configurar_rutas)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
