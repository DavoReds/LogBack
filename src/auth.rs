use argon2::{
    Argon2, Params,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

/// Genera un hash de una contraseña usando el algoritmo Argon2id
///
/// # Errors
///
/// Retorna un error si no puede generar el hash.
pub fn hash_clave(clave: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let params = Params::new(19_456, 2, 1, None)?;
    let hasher = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        params,
    );

    let hash = hasher.hash_password(clave.as_bytes(), &salt)?.to_string();

    Ok(hash)
}
