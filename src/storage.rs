use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorDBstore {
    pub id: i32,
    pub text: String,
    pub embedding: Vec<f32>,
}

impl VectorDBstore {
    pub fn new(id: i32, text: String, embedding: Vec<f32>) -> Self {
        Self {
            id,
            text,
            embedding,
        }
    }
}
