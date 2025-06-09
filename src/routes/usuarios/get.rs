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

            form action="/usuarios" method="POST" {
                fieldset {
                    label {
                        "Nombre"
                        input
                            type="text"
                            maxlength="100"
                            name="nombre"
                            autocomplete="username"
                            required;
                    }
                    label {
                        "Contraseña"
                        input
                            type="password"
                            name="clave"
                            autocomplete="new-password"
                            required;
                    }
                    label {
                        "Confirmar contraseña"
                        input
                            type="password"
                            name="confirmar"
                            autocomplete="off"
                            required;
                    }
                }
                input type="submit" value="Crear";
            }
        },
    ))
}
