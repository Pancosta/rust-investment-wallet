use askama::Template;
use axum::{
    Form, Router,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;

use crate::{
    app::AppState, auth::user::{UnauthenticatedUser, User}, error::AppError, models::{Asset, OwnedAsset}, repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
        .route("/assets", get(assets).post(purchase_asset))
        .route("/logout", get(logout))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(
    repository: Repository,
    jar: CookieJar,
    Form(request): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let unauth_user = UnauthenticatedUser::new(request.username, request.password);
    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExist) => unauth_user.register(&repository).await?,
        Err(other_err) => return Err(other_err),
    };

    let token = user.auth_token()?;
    let cookie = Cookie::build(("token", token)).http_only(true);

    Ok((jar.add(cookie), Redirect::to("/")))
}

pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    (jar.remove("token"), Redirect::to("/login"))
}

async fn index(maybe_user: Option<User>) -> Result<Redirect, AppError> {
    match maybe_user {
        Some(_) => Ok(Redirect::to("/assets")),
        None => Ok(Redirect::to("/login")),
    }
}

#[derive(Template)]
#[template(path = "assets.html")]
pub struct AssetsPage {
    pub owned_assets: Vec<OwnedAsset>,
    pub available_assets: Vec<Asset>,
    pub user: User,
}

pub async fn assets(repository: Repository, user: User) -> Result<Html<String>, AppError> {
    let owned_assets = repository.list_owned_assets(user.id()).await.unwrap();
    let available_assets = repository.list_assets().await.unwrap();

    let html = AssetsPage {
        owned_assets,
        available_assets,
        user,
    }
    .render()?;

    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct PurchaseAssetForm {
    pub asset_id: i64,
    pub unit_value: f64,
    pub quantity: f64,
}

pub async fn purchase_asset(
    repository: Repository,
    user: User,
    Form(request): Form<PurchaseAssetForm>,
) -> Result<Redirect, AppError> {
    repository
        .insert_owned_asset(
            user.id(),
            request.asset_id,
            request.quantity,
            request.unit_value,
        )
        .await
        .unwrap();

    Ok(Redirect::to("/assets"))
}

pub mod filters {
    use askama;
    use time::{
        OffsetDateTime,
        format_description::StaticFormatDescription,
        macros::format_description,
    };

    #[askama::filter_fn]
    pub fn human_datetime(
        datetime: &OffsetDateTime,
        _values: &dyn askama::Values,
    ) -> askama::Result<String> {
        const HUMAN_READABLE_FORMAT: StaticFormatDescription =
            format_description!(version = 2, "[year]-[month]-[day] [hour]:[minute]");

        datetime
            .format(&HUMAN_READABLE_FORMAT)
            .map_err(askama::Error::custom)
    }
}
