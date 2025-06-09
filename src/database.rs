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
