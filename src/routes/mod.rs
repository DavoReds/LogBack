mod index;

use actix_web::{Responder, web};
use index::index;

async fn ping() -> impl Responder {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    format!("{name} v{version}")
}

pub fn configurar_rutas(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
        .route("/ping", web::get().to(ping));
}
