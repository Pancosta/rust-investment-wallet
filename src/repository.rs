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

        let id = assets
            .iter()
            .map(|asset| asset.id)
            .max()
            .unwrap_or_default()
            + 1;

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

    pub async fn add_user(
        &self,
        username: &str,
        password_hash: &str,
    ) -> Result<UserRecord, crate::error::AppError> {
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

    pub async fn list_owned_assets(
        &self,
        user_id: i64,
    ) -> Result<Vec<crate::models::OwnedAsset>, Infallible> {
        let assets = self.state.assets.lock().await;
        let purchases = self.state.purchases.lock().await;
        let mut owned_assets = Vec::new();

        for asset in assets.iter() {
            let mut quantity_owned = 0.0;
            let mut value_delta = 0.0;
            let mut purchase_history = Vec::new();

            for purchase in purchases
                .iter()
                .filter(|p| p.user_id == user_id && p.asset_id == asset.id)
            {
                quantity_owned += purchase.quantity;
                let delta = (asset.unit_value - purchase.bought_for) * purchase.quantity;
                value_delta += delta;

                purchase_history.push(crate::models::PurchaseHistory {
                    bought_at: purchase.bought_at,
                    bought_for: purchase.bought_for,
                    quantity_bought: purchase.quantity,
                    value_delta: delta,
                });
            }

            if quantity_owned > 0.0 || !purchase_history.is_empty() {
                owned_assets.push(crate::models::OwnedAsset {
                    id: asset.id,
                    name: asset.name.clone(),
                    unit_value: asset.unit_value,
                    value_delta,
                    quantity_owned,
                    purchase_history,
                });
            }
        }

        Ok(owned_assets)
    }

    pub async fn insert_owned_asset(
        &self,
        user_id: i64,
        asset_id: i64,
        quantity: f64,
        unit_value: f64,
    ) -> Result<(), Infallible> {
        let mut purchases = self.state.purchases.lock().await;

        purchases.push(crate::models::PurchaseRecord {
            user_id,
            asset_id,
            quantity,
            bought_for: unit_value,
            bought_at: time::OffsetDateTime::now_utc(),
        });

        Ok(())
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
