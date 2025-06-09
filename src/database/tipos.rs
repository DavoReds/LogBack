use compact_str::CompactString;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct Tipo {
    pub id: Uuid,
    pub nombre: CompactString,
}

/// Devuelve todos los tipos de la base de datos.
pub async fn select_tipos(pool: &PgPool) -> Result<Vec<Tipo>, sqlx::Error> {
    sqlx::query_as!(Tipo, "SELECT id, nombre FROM tipos")
        .fetch_all(pool)
        .await
}
