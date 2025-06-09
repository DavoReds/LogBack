use actix_web::{Either, HttpResponse, web};
use maud::{Markup, html};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::templates::page_no_header;

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

    Either::Right(page_no_header(
        "Nuevo Usuario",
        html! {
            h1 { "Nuevo Usuario" }
        },
    ))
}
