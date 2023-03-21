mod config;
mod main_db_profile;
mod routers;
mod controllers;


use config::config_env;
use tide::{Result};



#[tokio::main]
async fn main()->Result<()> {
    config_env();
    let mut app=tide::new();
    app= routers::index::index_router(app).await;
    app.listen("127.0.0.1:8080").await?;
    return Ok(());
}
