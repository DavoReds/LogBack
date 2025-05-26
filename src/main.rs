use env_logger::Env;
use logback::{config::Settings, server::build_server};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let settings = Settings::new()?;

    let server = build_server(&settings)?;
    server.await?;

    Ok(())
}
