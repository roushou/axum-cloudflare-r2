mod storage;

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use eyre::Result;
use std::{net::SocketAddr, sync::Arc};
use storage::StorageObject;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::storage::{Storage, StorageConfig};

struct AppState {
    storage: Storage,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "axum_cloudflare_r2=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let storage = Storage::builder()
        .initialize(StorageConfig::load()?)
        .await?;

    let state = Arc::new(AppState { storage });

    let router = Router::new()
        .route("/_health", get(health))
        .route("/upload", post(upload_assets))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(address).await?;
    info!("Server running on {}", address);

    axum::serve(listener, router).await?;

    Ok(())
}

async fn health() -> &'static str {
    "healthy"
}

async fn upload_assets(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<(), StatusCode> {
    let mut storage_objects: Vec<StorageObject> = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        storage_objects.push(StorageObject {
            key: filename,
            data,
        })
    }

    for storage_object in storage_objects.into_iter() {
        state.storage.put_object(storage_object).await.unwrap();
    }

    Ok(())
}
