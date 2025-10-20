mod chunker;
mod embeddings;
mod routes;
mod storage;
use axum::{Router, routing::post};

use routes::{insert, query};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use storage::VectorDBstore;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let db: Arc<RwLock<Vec<VectorDBstore>>> = Arc::new(RwLock::new(Vec::new()));

    let app = Router::new()
        .route("/insert", post(insert))
        .route("/query", post(query))
        .with_state(db);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
