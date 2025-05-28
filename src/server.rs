use actix_web::{
    App, HttpServer,
    dev::Server,
    middleware::{Compress, Logger},
    web,
};
use anyhow::Context;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::TcpListener;

use crate::{
    config::{DatabaseSettings, Settings},
    routes::configurar_rutas,
};

/// Construye un servidor de `LogBack` basado en una configuración.
///
/// # Errors
///
/// La función retorna un error si falla en conectarse al socket establecido.
pub async fn build_server(cfg: &Settings) -> Result<Server, anyhow::Error> {
    let pool = web::Data::new(create_pool(&cfg.db).await?);

    let address = format!("{}:{}", cfg.server.host, cfg.server.port);
    let listener = TcpListener::bind(&address)?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%r %s %b %D"))
            .wrap(Compress::default())
            .configure(configurar_rutas)
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn create_pool(cfg: &DatabaseSettings) -> Result<PgPool, anyhow::Error> {
    let pool = PgPoolOptions::new()
        .connect_with(cfg.connect_options())
        .await
        .context("No se pudo establecer una conexión con la base de datos")?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .context("No se pudo ejecutar las migraciones necesarias")?;

    Ok(pool)
}
