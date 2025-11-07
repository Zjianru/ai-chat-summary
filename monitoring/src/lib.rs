use axum::{routing::get, Router};

pub async fn serve(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/metrics", get(|| async { "metrics_placeholder\n" }));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
