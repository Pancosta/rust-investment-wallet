use std::sync::Arc;
use axum::Router;

use tokio::{net::TcpListener, sync::Mutex};
use tracing::info;
use tracing_subscriber::{Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes;
use crate::models::Asset;

#[derive(Clone)]
pub struct AppState {
    pub assets: Arc<Mutex<Vec<Asset>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            assets: Default::default(),
        }
    }
}

pub struct App;

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();

        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        let router = Router::new()
            .nest("/api", routes::api::router())
            .with_state(AppState::new());

        info!("Server listening on http://0.0.0.0:3000");

        axum::serve(listener, router).await?;

        Ok(())
    }
}
