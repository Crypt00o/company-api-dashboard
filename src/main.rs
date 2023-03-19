mod config;
use config::config_env;





#[tokio::main]
async fn main() {
    config_env();
}
