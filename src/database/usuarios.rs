use compact_str::CompactString;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct Usuario {
    pub id: Uuid,
    pub nombre: CompactString,
    pub clave: CompactString,
}

/// Comprueba que exista al menos un usuario en la base de datos.
pub async fn comprobar_usuarios(pool: &PgPool) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM usuarios LIMIT 1)")
        .fetch_one(pool)
        .await
}
