use api;
use poem::{listener::TcpListener, Server};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = api::get_app();
    Server::new(TcpListener::bind("127.0.0.1:3000"))
    .run(app)
    .await

}
