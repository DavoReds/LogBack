use anyhow::Context;
use compact_str::CompactString;
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub db: DatabaseSettings,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub host: CompactString,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub user: CompactString,
    pub password: SecretBox<CompactString>,
    pub db: CompactString,
    pub host: CompactString,
    pub port: u16,
    pub ssl: bool,
}

impl DatabaseSettings {
    /// Crea un objeto que contiene las opciones de conexión a la base de datos.
    #[must_use]
    pub fn connect_options(&self) -> PgConnectOptions {
        let ssl = if self.ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.user)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl)
            .database(&self.db)
    }
}

impl Settings {
    /// Lee la configuración del archivo `config.toml` ubicado en el mismo
    /// directorio que el ejectuable de la aplicación y de las variables de
    /// entorno.
    ///
    /// # Errors
    ///
    /// La función retorna un error si no puede determinar el directorio
    /// actual, si no encuentra el archivo de configuración o si la
    /// configuración provista no puede ser deserializada correctamente.
    pub fn new() -> Result<Self, anyhow::Error> {
        let base_path = std::env::current_dir()
            .context("No se pudo determinar el directorio actual")?;

        let settings = config::Config::builder()
            .set_default("server.host", "localhost")?
            .set_default("server.port", 3000)?
            .set_default("db.host", "localhost")?
            .set_default("db.port", 5432)?
            .set_default("db.ssl", false)?
            .add_source(
                config::File::from(base_path.join("config.toml"))
                    .required(false),
            )
            .add_source(
                config::Environment::with_prefix("LOGBACK")
                    .prefix_separator("_")
                    .separator("_"),
            )
            .build()
            .context("No se pudo leer la configuración")?;

        settings
            .try_deserialize::<Self>()
            .context("La configuración no es válida")
    }
}
