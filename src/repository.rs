use std::convert::Infallible;

use axum::extract::FromRequestParts;

use crate::{
    app::AppState,
    models::{Asset, UserRecord},
};

pub struct Repository {
    state: AppState,
}

impl Repository {
    pub async fn list_assets(&self) -> Result<Vec<Asset>, Infallible> {
        let assets = self.state.assets.lock().await;
        Ok(assets.clone())
    }

    pub async fn create_asset(&self, name: String, unit_value: f64) -> Result<Asset, Infallible> {
        let mut assets = self.state.assets.lock().await;

        let id = assets.iter().map(|asset| asset.id).max().unwrap_or_default() + 1;

        let new_asset = Asset {
            id,
            name,
            unit_value,
        };

        assets.push(new_asset.clone());
        Ok(new_asset)
    }

    pub async fn update_asset(
        &self,
        asset_id: i64,
        name: Option<String>,
        unit_value: Option<f64>,
    ) -> Result<Option<Asset>, Infallible> {
        let mut assets = self.state.assets.lock().await;

        for asset in assets.iter_mut() {
            if asset.id == asset_id {
                if let Some(new_name) = name {
                    asset.name = new_name;
                }
                if let Some(new_value) = unit_value {
                    asset.unit_value = new_value;
                }
                return Ok(Some(asset.clone()));
            }
        }
        
        Ok(None)
    }

    pub async fn add_user(&self, username: &str, password_hash: &str) -> Result<UserRecord, crate::error::AppError> {
        let mut users = self.state.users.lock().await;

        if users.iter().any(|u| u.username == username) {
            return Err(crate::error::AppError::UsernameTaken);
        }

        let id = users.iter().map(|user| user.id).max().unwrap_or_default() + 1;

        let new_user = UserRecord {
            id,
            username: username.to_string(),
            password_hash: password_hash.to_string(),
        };

        users.push(new_user.clone());
        Ok(new_user)
    }

    pub async fn get_user_by_name(&self, username: &str) -> Result<Option<UserRecord>, Infallible> {
        let users = self.state.users.lock().await;
        
        Ok(users.iter().find(|u| u.username == username).cloned())
    }
}

impl FromRequestParts<AppState> for Repository {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            state: state.clone(),
        })
    }
}
