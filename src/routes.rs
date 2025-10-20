// Code for handling routes /insert and /query
use crate::chunker::chunk_text;
use crate::embeddings::get_embeddings;
use crate::storage::VectorDBstore;
use axum::{Json, extract::State, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsertData {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsertResponse {
    pub status: String,
}

pub async fn insert(
    db: State<Arc<RwLock<Vec<VectorDBstore>>>>,
    data: Json<InsertData>,
) -> impl IntoResponse {
    let chunks = chunk_text(data.text.as_str(), 300, 20).unwrap();
    let chunk_refs: Vec<&str> = chunks.iter().map(|s| s.as_str()).collect();
    let embeddings = get_embeddings(chunk_refs).unwrap();
    for i in 0..embeddings.len() {
        let item = VectorDBstore::new(i as i32, chunks[i].clone(), embeddings[i].clone());
        {
            let mut db = db.write().unwrap();
            db.push(item.clone());
            println!("{:?}\n", db[i]);
        }
    }

    Json(InsertResponse {
        status: "Inserted".to_string(),
    })
}
