use uuid::Uuid;
use crate::models::{TrainResponse, InferResponse};
use actix_web::{web, post, Responder};

#[post("/train")]
pub async fn train() -> impl Responder {
    let uuid = Uuid::new_v4();
    web::Json(
        TrainResponse { 
            model: String::from("dummy"),
            task: uuid,
            state: String::from("pending")
        })
}

#[post("/infer")]
pub async fn infer() -> impl Responder {
    web::Json(
        InferResponse { 
            model: String::from("dummy"),
            value: 0.0
        })
}