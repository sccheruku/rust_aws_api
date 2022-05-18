use poem::{ Route};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}

pub fn get_app() -> Route {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    Route::new().nest("/api", api_service).nest("/", ui)
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

