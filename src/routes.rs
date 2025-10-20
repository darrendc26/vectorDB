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
    let chunks = chunk_text(data.text.as_str(), 50, 10).unwrap();
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
        status: "Inserted into VectorDB".to_string(),
    })
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryData {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryResponse {
    pub text: String,
    pub similarity: f32,
}

pub async fn query(
    db: State<Arc<RwLock<Vec<VectorDBstore>>>>,
    data: Json<QueryData>,
) -> impl IntoResponse {
    let query_embedding = get_embeddings(vec![data.text.as_str()]).unwrap()[0].clone();
    let db = db.read().unwrap();
    let mut similar: Vec<(f32, String)> = Vec::new();
    for i in 0..db.len() {
        let similarity = cosine_similarity(&query_embedding, &db[i].embedding);
        similar.push((similarity, db[i].text.clone()));
    }

    similar.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let results: Vec<QueryResponse> = similar
        .into_iter()
        .take(5)
        .map(|s| QueryResponse {
            text: s.1,
            similarity: s.0,
        })
        .collect();

    Json(results)
}

// cosine_similarity(A,B)= A.B/(∥A∥×∥B∥)
// ∥A∥=Sqrt(Sum(Ai2))
// ∥B∥=Sqrt(Sum(Bi2))
// A.B=Sum(Ai2*Bi2)
fn cosine_similarity(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    let dot_product = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>();
    let norm_a = a.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();

    dot_product / (norm_a * norm_b)
}
