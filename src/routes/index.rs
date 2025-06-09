use actix_web::{ResponseError, web};
use maud::{Markup, html};
use sqlx::PgPool;
use thiserror::Error;

use crate::{
    database::{select_estados, select_tipos},
    templates::page,
};

#[derive(Debug, Error)]
pub enum IndexError {
    #[error("Algo salió mal")]
    UnexpectedError(#[from] sqlx::Error),
}

impl ResponseError for IndexError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn index(pool: web::Data<PgPool>) -> Result<Markup, IndexError> {
    let tipos = select_tipos(pool.as_ref()).await?;
    let estados = select_estados(pool.as_ref()).await?;

    Ok(page(
        "Principal",
        html! {
            article {
                header {
                    h3 { "Nueva Entrada" }
                }

                form
                    hx-post="/entradas"
                    hx-swap="none"
                    "hx-on::after-request"="if(event.detail.successful) this.reset()"
                {
                    label {
                        "Nombre"
                        input name="nombre" type="text" maxlength="100" required;
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

            div
                hx-get="/entradas"
                hx-trigger="load, buscar_entradas from:body"
                hx-swap="innerHTML"
            {
                img .htmx-indicator alt="Cargando entradas..." src="/public/img/loader.svg";
            }
        },
    ))
}
