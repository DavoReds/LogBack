use actix_web::{Responder, web};

async fn ping() -> impl Responder {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    format!("{name} v{version}")
}

pub fn configurar_rutas(cfg: &mut web::ServiceConfig) {
    cfg.route("/ping", web::get().to(ping));
}
