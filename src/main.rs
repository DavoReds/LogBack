use logback::{config::Settings, server::build_server};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::try_init_timed_custom_env("LOGBACK_LOG")?;

    let settings = Settings::new()?;

    let server = build_server(&settings).await?;
    server.await?;

    Ok(())
}
