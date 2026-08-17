mod app;
pub mod models;
use crate::app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
