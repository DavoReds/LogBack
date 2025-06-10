use actix_web::{Either, HttpResponse, web};
use maud::Markup;
use std::sync::atomic::{AtomicBool, Ordering};

use super::formulario_nuevo_usuario;

pub async fn get_nuevo_usuario(
    existe: web::Data<AtomicBool>,
) -> Either<HttpResponse, Markup> {
    if existe.load(Ordering::Relaxed) {
        return Either::Left(
            HttpResponse::Found()
                .insert_header(("Location", "/"))
                .finish(),
        );
    }

    Either::Right(formulario_nuevo_usuario(None))
}
