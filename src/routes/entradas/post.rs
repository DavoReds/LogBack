use actix_web::{HttpResponse, Responder, web};
use chrono::Local;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PostEntradasInput {
    pub nombre: String,
    pub tipo: Uuid,
    pub estado: Uuid,
}

pub async fn post_entradas(
    pool: web::Data<PgPool>,
    form: web::Form<PostEntradasInput>,
) -> impl Responder {
    let fecha = Local::now().date_naive();
    let id = Uuid::now_v7();

    if let Err(e) = sqlx::query!(
        r"INSERT INTO entradas (id, nombre, fecha, tipo, estado)
        VALUES ($1, $2, $3, $4, $5)",
        id,
        &form.nombre,
        fecha,
        form.tipo,
        form.estado
    )
    .execute(pool.as_ref())
    .await
    {
        log::error!("{e}");
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Created()
        .insert_header(("HX-Trigger", "buscar_entradas"))
        .body("¡Creado!")
}
