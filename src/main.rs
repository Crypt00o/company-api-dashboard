mod config;
mod main_db_profile;
use config::config_env;



#[tokio::main]
async fn main() {
    config_env();

}
