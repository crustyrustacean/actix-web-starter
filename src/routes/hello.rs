// src/routes/hello.rs

use actix_web::web::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    message: String,
}

pub async fn hello() -> Json<Message> {
    Json(Message {
        message: "Hello from Rust on Vercel".to_string(),
    })
}
