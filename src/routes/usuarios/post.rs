use actix_web::{Either, HttpResponse, web};
use compact_str::CompactString;
use maud::Markup;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::atomic::{AtomicBool, Ordering};
use uuid::Uuid;

use super::formulario_nuevo_usuario;
use crate::{auth::hash_clave, routes::usuarios::ErrorNuevoUsuario};

#[derive(Debug, Deserialize)]
pub struct PostUsuarioInput {
    nombre: CompactString,
    clave: CompactString,
    confirmar: CompactString,
}

pub async fn post_usuario(
    pool: web::Data<PgPool>,
    form: web::Form<PostUsuarioInput>,
    usuario_existe: web::Data<AtomicBool>,
) -> Either<Markup, HttpResponse> {
    let PostUsuarioInput {
        nombre,
        clave,
        confirmar,
    } = form.into_inner();

    if nombre.len() > 100 {
        return Either::Left(formulario_nuevo_usuario(Some(
            ErrorNuevoUsuario::Nombre(
                "El nombre no puede ser más largo que 100 caracteres".into(),
            ),
        )));
    }

    if clave != confirmar {
        return Either::Left(formulario_nuevo_usuario(Some(
            ErrorNuevoUsuario::Clave("Las contraseñas no concuerdan".into()),
        )));
    }

    let hash = match hash_clave(clave.as_str()) {
        Ok(hash) => hash,
        Err(e) => {
            log::error!("Error computando hash de la contraseña: {e}");

            return Either::Left(formulario_nuevo_usuario(Some(
                ErrorNuevoUsuario::Clave("Error inesperado".into()),
            )));
        }
    };

    if let Err(e) = sqlx::query!(
        "INSERT INTO usuarios (id, nombre, clave) VALUES ($1, $2, $3)",
        Uuid::now_v7(),
        nombre.as_str(),
        hash
    )
    .execute(pool.as_ref())
    .await
    {
        log::error!("Error con la base de datos: {e}");

        return Either::Right(HttpResponse::InternalServerError().finish());
    }

    usuario_existe.store(true, Ordering::Relaxed);

    Either::Right(
        HttpResponse::SeeOther()
            .insert_header(("Location", "/"))
            .finish(),
    )
}
