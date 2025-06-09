mod entradas;
mod index;
mod usuarios;

use actix_web::{Responder, middleware::from_fn, web};

use crate::{
    middleware::existen_usuarios,
    routes::{
        entradas::{
            delete_entrada, get_entradas, get_formulario_entrada,
            post_entradas, put_entrada,
        },
        index::index,
        usuarios::get_nuevo_usuario,
    },
};

pub fn configurar_rutas(cfg: &mut web::ServiceConfig) {
    cfg.route("/ping", web::get().to(ping))
        .route("/usuarios", web::get().to(get_nuevo_usuario));

    cfg.service(
        web::scope("")
            .wrap(from_fn(existen_usuarios))
            .route("/", web::get().to(index))
            .service(
                web::scope("/entradas")
                    .route("", web::get().to(get_entradas))
                    .route("", web::post().to(post_entradas))
                    .service(
                        web::scope("/{id}")
                            .route("", web::put().to(put_entrada))
                            .route("", web::delete().to(delete_entrada))
                            .route(
                                "/editar",
                                web::get().to(get_formulario_entrada),
                            ),
                    ),
            ),
    );
}

async fn ping() -> impl Responder {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    format!("{name} v{version}")
}
