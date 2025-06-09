use compact_str::CompactString;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct Estado {
    pub id: Uuid,
    pub nombre: CompactString,
}

/// Devuelve todos los estados de la base de datos.
pub async fn select_estados(pool: &PgPool) -> Result<Vec<Estado>, sqlx::Error> {
    sqlx::query_as!(Estado, "SELECT id, nombre FROM estados")
        .fetch_all(pool)
        .await
}
