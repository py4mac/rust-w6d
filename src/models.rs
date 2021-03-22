use uuid::Uuid;
use serde::{Serialize};

#[derive(Serialize)]
pub struct TrainResponse {
    pub model: String,
    pub task: Uuid,
    pub state: String,
}

#[derive(Serialize)]
pub struct InferResponse {
    pub model: String,
    pub value: f64,
}