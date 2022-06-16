use poem::{Route, Result};
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    OpenApi, OpenApiService, ApiResponse,
    Object
};

struct Api;

#[derive(Object)]
struct Person {
    first_name: String,
    last_name: String,
    extra: Option<serde_json::Value>,
}

#[derive(ApiResponse)]
enum GetResponse{
    #[oai(status = 200)]
    Person(Json<Person>)
}

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }

    #[oai(path = "/person", method = "get")]
    async fn person(&self, name: Query<Option<String>>) -> Result<GetResponse> {
        // match name.0 {
        //     Some(name) => PlainText(format!("hello, {}!", name)),
        //     None => PlainText("hello!".to_string()),
        // }
        Ok(GetResponse::Person(Json(Person {
            first_name: "hello".into(),
            last_name: "last_name".into(),
            extra: Some(serde_json::json!({
                "extra_prop": "hello extra prop"
            })),
        })))
    }

    #[oai(path = "/person", method = "post")]
    async fn post_person(&self, person: Json<Person>) -> PlainText<String> {
        PlainText("hello!".to_string())
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
