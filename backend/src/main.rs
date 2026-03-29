use clap::Parser;
use chronicle_engine_rs::{build_app_with_config_path, config::AppConfig, init_logging};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "chronicle-engine-rs")]
struct Args {
    #[arg(long, default_value = "/etc/chronicle-engine-backend/backend.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = AppConfig::load(&args.config)?;

    // Initialize tracing/logging from config before anything else.
    init_logging(&config.logging.level);

    let bind = config.server.bind.clone();
    let app = build_app_with_config_path(config, Some(args.config.clone()))?;

    let listener = tokio::net::TcpListener::bind(&bind).await?;
    tracing::info!("chronicle-engine-rs listening on {bind}");
    axum::serve(listener, app).await?;
    Ok(())
}
