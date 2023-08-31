mod pages;
use std::sync::Arc;

use anyhow::Context;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use pages::fortune::fortune;
use pages::next_prime::{calculate_next_prime, view, PrimeState};

struct AppState {
    counter: Arc<PrimeState>,
}

fn api_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/next-prime",
            post(calculate_next_prime).with_state(state.counter),
        )
        .route("/fortune", get(fortune))
}

fn router(state: AppState) -> anyhow::Result<Router> {
    let assets_path = std::env::current_dir()?;
    let assets_serve_dir = ServeDir::new(format!(
        "{}/assets",
        assets_path.to_str().expect("assets path is not valid utf8")
    ));

    Ok(Router::new()
        .route("/", get(view).with_state(state.counter.clone()))
        .nest("/api", api_router(state))
        .nest_service("/assets", assets_serve_dir))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_static_web_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router and assets");

    // run it, make sure you handle parsing your environment variables properly!
    let port = std::env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let port = port
        .parse()
        .context("PORT environment variable is not a valid u16")?;

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));

    info!("router initialized, now listening on port {}", port);

    let state = AppState {
        counter: Arc::new(PrimeState::new()),
    };

    axum::Server::bind(&addr)
        .serve(router(state)?.into_make_service())
        .await
        .context("error while starting API server")?;

    Ok(())
}
