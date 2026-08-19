use axum::{Json, Router, extract::State, routing::get};

use crate::{app::AppState, models::Asset};

pub fn router() -> Router<AppState> {
    Router::new().route("/assets", get(list_assets).post(create_asset))
}


#[tracing::instrument(skip_all)]
async fn list_assets(state: State<AppState>) -> Json<Vec<Asset>> {
    let assets = state.assets.lock().await;
    Json(assets.clone())
}

#[derive(serde::Deserialize)]
struct CreateAssetRequest {
    name: String,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
async fn create_asset(state: State<AppState>, Json(request): Json<CreateAssetRequest>) -> Json<Asset> {
    let mut assets = state.assets.lock().await;

    let id = assets.iter().map(|asset| asset.id).max().unwrap_or_default() + 1;

    let new_asset = Asset {
        id,
        name: request.name,
        unit_value: request.unit_value,
    };

    assets.push(new_asset.clone());

    Json(new_asset)
}