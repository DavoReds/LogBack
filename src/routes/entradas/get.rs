use actix_web::{
    HttpRequest, HttpResponse, ResponseError, http::header::ContentType, web,
};
use compact_str::CompactString;
use maud::{Markup, html};
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use crate::templates::page;

#[derive(Debug)]
struct Entrada {
    id: Uuid,
    nombre: CompactString,
    tipo: CompactString,
    color_tipo: CompactString,
    estado: CompactString,
    color_estado: CompactString,
}

#[allow(clippy::future_not_send)]
pub async fn get_entradas(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<Markup, GetEntradasError> {
    let entradas = sqlx::query_as!(
        Entrada,
        r"SELECT
        e.id, e.nombre, t.nombre as tipo, t.color AS color_tipo,
        s.nombre as estado, s.color AS color_estado
        FROM entradas e
        JOIN tipos t ON (e.tipo = t.id)
        JOIN estados s ON (e.estado = s.id)
        ORDER BY t.nombre"
    )
    .fetch_all(pool.as_ref())
    .await?;

    let just_table = req.headers().get("HX-Request").is_some()
        && req.headers().get("HX-Boosted").is_none();

    if just_table {
        return Ok(tabla_entradas(&entradas));
    }

    Ok(page("Entradas", tabla_entradas(&entradas)))
}

#[derive(Debug, Error)]
pub enum GetEntradasError {
    #[error("Algo salió mal")]
    Inesperado(#[from] sqlx::Error),
}

impl ResponseError for GetEntradasError {
    fn error_response(
        &self,
    ) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        log::error!("Error interactuando con la base de datos: {self:?}");

        HttpResponse::Ok().content_type(ContentType::html()).body(page(
                "Entradas",
                html! {
                    section #entradas {
                        h1 { "Entradas" }
                        p { "Error trayendo referencias. Por favor intente nuevamente." }
                    }
                },
            ))
    }
}

fn tabla_entradas(entradas: &[Entrada]) -> Markup {
    html! {
        section #entradas
         {
            h1 { "Entradas" }

            table .striped {
                thead {
                    tr {
                        th scope="col" { "Nombre" }
                        th scope="col" { "Tipo" }
                        th scope="col" { "Estado" }
                        th scope="col" { "Acciones" }
                    }
                }
                tbody {
                    @for entrada in entradas {
                        tr {
                            td { (entrada.nombre) }
                            td style={"color:" (entrada.color_tipo)} { (entrada.tipo) }
                            td style={"color:" (entrada.color_estado)} { (entrada.estado) }
                            td {
                                div.grid {
                                    button .contrast data-tooltip="Editar" {
                                        img width="16" alt="Editar" src="/public/img/lapiz.svg";
                                    }
                                    button
                                        data-tooltip="Eliminar"
                                        hx-delete={"/entradas/" (entrada.id)}
                                        hx-swap="none"
                                    {
                                        img width="16" alt="Eliminar" src="/public/img/cross.svg";
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
