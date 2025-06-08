use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete_entrada(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    let id = id.into_inner();

    log::info!("Borrando entrada {id}");

    if let Err(e) = sqlx::query!("DELETE FROM entradas WHERE id = $1", id)
        .execute(pool.as_ref())
        .await
    {
        log::error!("{e}");
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok()
        .insert_header(("HX-Trigger", "buscar_entradas"))
        .finish()
}
