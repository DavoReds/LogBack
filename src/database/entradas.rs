use compact_str::CompactString;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct Entrada {
    pub id: Uuid,
    pub nombre: CompactString,
    pub tipo: Uuid,
    pub estado: Uuid,
}

/// Retorna una entrada de la base de datos.
pub async fn select_entrada(
    pool: &PgPool,
    id: &Uuid,
) -> Result<Entrada, sqlx::Error> {
    sqlx::query_as!(
        Entrada,
        "SELECT id, nombre, tipo, estado from entradas WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}
