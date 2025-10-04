use std::io::Result;

use action::{route, CONFIG};
use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(route::index))
        .route("/jobs", get(route::jobs))
        .route("/jobs/list", get(route::list_jobs))
        .route("/jobs/upload", post(route::upload_job))
        .route("/jobs/{id}/run", post(route::run_job))
        .route("/artifacts", get(route::artifacts))
        .route("/artifacts/list", get(route::list_artifacts))
        .route("/artifacts/{id}", get(route::download_artifact))
        .route("/clean", post(route::clean));

    let addr = format!("{}:{}", CONFIG.app.host, CONFIG.app.port);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
