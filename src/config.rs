use anyhow::Context;
use compact_str::CompactString;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub host: CompactString,
    pub port: u16,
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
        let base_path =
            std::env::current_dir().context("No se pudo determinar el directorio actual")?;

        let settings = config::Config::builder()
            .set_default("server.host", "localhost")?
            .set_default("server.port", 3000)?
            .add_source(config::File::from(base_path.join("config.toml")))
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
