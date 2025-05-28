use actix_web::web;
use maud::{Markup, html};
use sqlx::PgPool;

use crate::templates::page;

pub async fn index(pool: web::Data<PgPool>) -> Result<Markup, actix_web::Error> {
    let estados = sqlx::query!("SELECT id, nombre, color FROM estados")
        .fetch_all(pool.as_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Algo salió mal"))?;

    Ok(page(
        "Principal",
        html! {
            h1 { "LogBack" }

            h2 { "Crear Nueva Entrada" }

            form {
                fieldset {
                    label {
                        "Nombre"
                        input name="nombre" type="text";
                    }
                }
            }

            ul {
                @for estado in estados {
                    li style={"color:" (estado.color)}{
                        (estado.id)
                        " - "
                        (estado.nombre)
                    }
                }
            }
        },
    ))
}
