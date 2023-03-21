mod config;
mod main_db_profile;
use std::error::Error;

use config::config_env;
use tide;

#[tokio::main]
async fn main()->tide::Result<()> {
    config_env();
    let mut app=tide::new();
    app.listen("127.0.0.1:8080").await?;
    return Ok(());
}
