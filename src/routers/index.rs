
use tide::Server;
use super::super::controllers::main_controller::hello_client;

pub async fn index_router(mut app: Server<()>)->Server<()>{
app.at("/*").all(hello_client);
    return app;
}
