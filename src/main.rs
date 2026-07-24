use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use vocab_api::{AppState, config::Config, routes};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::load()?;
    let addr = config.addr.clone();

    let state = AppState::new(config);

    let app = Router::new()
        .nest("/api", routes::router())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(addr).await?;
    info!("listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");
}
