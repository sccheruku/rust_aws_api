use api;
use poem_lambda::{run, Error};
use poem::Route;

#[tokio::main]
async fn main() -> Result<(), Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    // tracing_subscriber::fmt::init();

    // prod is the stage name that prepends the path requested via API Gateway
    let app = Route::new().nest("/prod", api::get_app()); 
    run(app).await
}
