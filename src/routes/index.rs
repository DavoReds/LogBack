use actix_web::web;
use maud::{Markup, html};
use sqlx::PgPool;

use crate::templates::page;

pub async fn index(pool: web::Data<PgPool>) -> Result<Markup, actix_web::Error> {
    let estados = sqlx::query!("SELECT id, nombre, color FROM estados")
        .fetch_all(pool.as_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Algo salió mal"))?;

    let tipos = sqlx::query!("SELECT id, nombre, color FROM tipos")
        .fetch_all(pool.as_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Algo salió mal"))?;

    Ok(page(
        "Principal",
        html! {
            h1 { "LogBack" }

            section {
                h2 { "Nueva Entrada" }

                form action="/entradas" method="POST" {
                    label {
                        "Nombre"
                        input name="nombre" type="text" required;
                    }
                    fieldset .grid {
                        label {
                            "Tipo"
                            select name="tipo" aria-label="Tipo" required {
                                @for tipo in tipos {
                                    option value=(tipo.id) {(tipo.nombre)}
                                }
                            }
                        }
                        label {
                            "Estado"
                            select name="estado" aria-label="Estado" required {
                                @for estado in estados {
                                    option value=(estado.id) {(estado.nombre)}
                                }
                            }
                        }
                    }
                    input type="submit" value="Crear";
                }
            }
            }
        },
    ))
}
