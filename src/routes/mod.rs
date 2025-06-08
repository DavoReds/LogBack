mod entradas;
mod index;

use actix_web::{Responder, web};

use crate::routes::{
    entradas::{delete_entrada, get_entradas, post_entradas},
    index::index,
};

pub fn configurar_rutas(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
        .route("/ping", web::get().to(ping))
        .service(
            web::resource("/entradas")
                .get(get_entradas)
                .post(post_entradas),
        )
        .service(web::resource("/entradas/{id}").delete(delete_entrada));
}

async fn ping() -> impl Responder {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    format!("{name} v{version}")
}
