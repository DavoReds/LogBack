use actix_web::{HttpResponse, ResponseError, web};
use compact_str::CompactString;
use maud::{Markup, html};
use serde::Deserialize;
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use crate::database::{
    entradas::select_entrada, estados::select_estados, tipos::select_tipos,
};

pub async fn get_formulario_entrada(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> Result<Markup, GetFormularioEntradaError> {
    let entrada = select_entrada(pool.as_ref(), id.as_ref()).await?;
    let tipos = select_tipos(pool.as_ref()).await?;
    let estados = select_estados(pool.as_ref()).await?;

    Ok(html! {
        tr {
            td {
                input
                    form=(entrada.id)
                    type="text"
                    name="nombre"
                    aria-label="Nombre"
                    maxlength="100"
                    value=(entrada.nombre)
                    required;
            }
            td {
                select
                    form=(entrada.id)
                    name="tipo"
                    aria-label="Tipo"
                    required
                {
                    @for tipo in tipos {
                        option
                            value=(tipo.id)
                            selected[tipo.id == entrada.tipo]
                        { (tipo.nombre) }
                    }
                }
            }
            td {
                select
                    form=(entrada.id)
                    name="estado"
                    aria-label="Estado"
                    required
                {
                    @for estado in estados {
                        option
                            value=(estado.id)
                            selected[estado.id == entrada.estado]
                        { (estado.nombre) }
                    }
                }
            }
            td {
                button
                    hx-put={"/entradas/" (entrada.id)}
                    hx-target="closest tr"
                    hx-include="closest tr"
                    hx-swap="outerHTML"
                {
                    "Enviar"
                }
            }
        }
    })
}

#[derive(Debug, Error)]
pub enum GetFormularioEntradaError {
    #[error("Algo salió mal")]
    Inesperado(#[from] sqlx::Error),
}

impl ResponseError for GetFormularioEntradaError {
    fn error_response(
        &self,
    ) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        log::error!("Error en la base de datos: {self:?}");

        if let Self::Inesperado(sqlx::Error::RowNotFound) = self {
            HttpResponse::NotFound().finish()
        } else {
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PutEntradaInput {
    pub nombre: CompactString,
    pub tipo: Uuid,
    pub estado: Uuid,
}

pub async fn put_entrada(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    form: web::Form<PutEntradaInput>,
) -> Result<Markup, PutEntradaError> {
    sqlx::query!(
        "UPDATE entradas SET nombre = $1, tipo = $2, estado = $3 WHERE id = $4",
        form.nombre.as_str(),
        form.tipo,
        form.estado,
        id.as_ref()
    )
    .execute(pool.as_ref())
    .await?;

    let tipo = sqlx::query!(
        "SELECT nombre, color FROM tipos WHERE id = $1",
        form.tipo
    )
    .fetch_one(pool.as_ref())
    .await?;

    let estado = sqlx::query!(
        "SELECT nombre, color FROM estados WHERE id = $1",
        form.estado
    )
    .fetch_one(pool.as_ref())
    .await?;

    Ok(html! {
        tr {
            td { (form.nombre) }
            td style={"color:" (tipo.color)} { (tipo.nombre) }
            td style={"color:" (estado.color)} { (estado.nombre) }
            td {
                div.grid {
                    button
                        .contrast
                        data-tooltip="Editar"
                        hx-get={"/entradas/" (id) "/editar"}
                        hx-target="closest tr"
                        hx-swap="outerHTML"
                    {
                        img width="16" alt="Editar" src="/public/img/lapiz.svg";
                    }

                    button
                        data-tooltip="Eliminar"
                        hx-delete={"/entradas/" (id)}
                        hx-swap="none"
                    {
                        img width="16" alt="Eliminar" src="/public/img/cross.svg";
                    }
                }
            }
        }
    })
}

#[derive(Debug, Error)]
pub enum PutEntradaError {
    #[error("Algo salió mal")]
    Inesperado(#[from] sqlx::Error),
}

impl ResponseError for PutEntradaError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        log::error!("Error en la base de datos: {self:?}");

        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}
