use anyhow::Result;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "ferrimon-web")]
#[command(about = "Web server to view collected metrics")]
struct Args {
    #[arg(short, long, default_value = "./data")]
    workdir: PathBuf,

    #[arg(short, long, default_value = "8080")]
    port: u16,
}

struct AppState {
    workdir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    info!(
        workdir = %args.workdir.display(),
        port = args.port,
        "Starting ferrimon web server"
    );

    let state = Arc::new(AppState {
        workdir: args.workdir.clone(),
    });

    let app = Router::new()
        .route("/", get(index))
        .route("/metrics.csv", get(serve_csv))
        .route("/metrics.ndjson", get(serve_ndjson))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!(addr = %addr, "Server listening");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn index() -> Html<String> {
    Html(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Ferrimon Metrics</title>
</head>
<body>
    <h1>Ferrimon Metrics</h1>
    <h2>Download Metrics</h2>
    <ul>
        <li><a href="/metrics.csv">metrics.csv</a> - CSV format</li>
        <li><a href="/metrics.ndjson">metrics.ndjson</a> - NDJSON format</li>
    </ul>
</body>
</html>"#
            .to_string(),
    )
}

async fn serve_csv(state: axum::extract::State<Arc<AppState>>) -> impl IntoResponse {
    let path = state.0.workdir.join("metrics.csv");
    match tokio::fs::read(&path).await {
        Ok(content) => {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                "text/csv".parse().unwrap(),
            );
            headers.insert(
                axum::http::header::CONTENT_DISPOSITION,
                "attachment; filename=\"metrics.csv\"".parse().unwrap(),
            );
            (axum::http::StatusCode::OK, headers, content)
        }
        Err(e) => {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                "text/plain".parse().unwrap(),
            );
            (
                axum::http::StatusCode::NOT_FOUND,
                headers,
                format!("CSV file not found: {}", e).into_bytes(),
            )
        }
    }
}

async fn serve_ndjson(state: axum::extract::State<Arc<AppState>>) -> impl IntoResponse {
    let path = state.0.workdir.join("metrics.ndjson");
    match tokio::fs::read(&path).await {
        Ok(content) => {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                "application/x-ndjson".parse().unwrap(),
            );
            headers.insert(
                axum::http::header::CONTENT_DISPOSITION,
                "attachment; filename=\"metrics.ndjson\"".parse().unwrap(),
            );
            (axum::http::StatusCode::OK, headers, content)
        }
        Err(e) => {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                "text/plain".parse().unwrap(),
            );
            (
                axum::http::StatusCode::NOT_FOUND,
                headers,
                format!("NDJSON file not found: {}", e).into_bytes(),
            )
        }
    }
}
