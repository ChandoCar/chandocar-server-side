use actix_web::get;
use actix_web::web::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

#[get("/hello_world")]
pub async fn hello_world() -> Json<Message> {
    Json(Message {
        message: "Hello, world!".to_string(),
    })
}
