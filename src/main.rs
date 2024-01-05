use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct AddRequest {
    a: i32,
    b: i32,
}

#[derive(Serialize)]
struct AddResponse {
    result: i32,
}

async fn add_numbers(req: AddRequest) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&AddResponse {
        result: req.a + req.b,
    }))
}

#[tokio::main]
async fn main() {
    let add_route = warp::post()
        .and(warp::path("add"))
        .and(warp::body::json())
        .and_then(add_numbers);

    warp::serve(add_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
