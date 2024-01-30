use rocket::{post, serde::json::Json};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema )]
#[serde(crate = "rocket::serde")]
struct AddRequest {
    a: i32,
    b: i32,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct AddResponse {
    result: i32,
}

#[openapi(tag = "Addition")]
#[post("/add", format = "json", data = "<input>")]
fn add(input: Json<AddRequest>) -> Json<AddResponse> {
    let sum = input.a + input.b;
    Json(AddResponse { result: sum })
}

#[rocket::main]
async fn main() {

    let launch_result = rocket::build()
        .mount(
            "/", 
            openapi_get_routes![
                add
                ],
        )
        .mount(
            "/swagger-ui",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
               }),
        )
        .launch()
        .await;   
    match launch_result {
        Ok(_) => println!("Rocket launched successfully"),
        Err(e) => println!("Error launching Rocket: {}", e),
    };
}

