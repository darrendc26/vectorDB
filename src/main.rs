mod chunker;
mod embeddings;
mod routes;
mod storage;
use axum::{Router, routing::post};

use routes::insert;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use storage::VectorDBstore;
use tokio::net::TcpListener;

// use crate::chunker::chunk_text;
// use crate::embeddings::get_embeddings;
// fn main() {
//     let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin vehicula, sapien ac fermentum varius, libero justo faucibus purus, a bibendum sapien ligula vel justo. Integer blandit felis sed libero ultrices, id eleifend magna hendrerit. Sed pretium, justo at malesuada tincidunt, urna urna tincidunt mauris, nec dignissim justo nulla at nulla. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.

// Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Integer vel libero id purus luctus fringilla. Curabitur ac leo sit amet enim tincidunt bibendum. Vivamus imperdiet neque et nulla elementum, at sollicitudin lectus viverra. Maecenas sed est a tortor porttitor sagittis. Sed in libero ut neque sodales elementum.

// Phasellus luctus ligula in metus imperdiet, non congue eros sagittis. Fusce euismod leo ut est pharetra, ut fermentum nulla vulputate. Aenean nec sapien sit amet justo vestibulum ultrices. In hac habitasse platea dictumst. Nulla facilisi. Curabitur et ex a libero consequat consequat. Donec at ligula nec nulla scelerisque ultricies.

// Morbi vitae sapien non nisl tempus convallis a nec lectus. Cras in libero id purus pretium tincidunt. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Praesent sagittis risus vitae risus dapibus, in fermentum erat blandit. Duis imperdiet eros at felis vulputate, nec fringilla nunc hendrerit.

// Integer cursus, purus non ullamcorper tincidunt, nulla lorem laoreet arcu, id facilisis ex erat sed nunc. Sed sit amet purus at nulla vulputate pulvinar. Nunc ac est id urna mattis viverra. Nam ut purus at nibh convallis fermentum. Donec a ligula nec odio dapibus malesuada vel ut libero. Etiam a sapien nec dolor mollis dapibus.

// Aliquam erat volutpat. Vestibulum suscipit erat sed ex dapibus, a fermentum lorem eleifend. Donec fermentum sem at diam efficitur, nec bibendum ligula tempor. Cras faucibus, augue vel vehicula vulputate, nisl urna fermentum mauris, vel posuere odio urna id arcu. Quisque lacinia justo a augue elementum, vitae vehicula augue sagittis.

// Sed rhoncus libero non ante vulputate, ut vehicula libero bibendum. Praesent euismod magna a felis scelerisque, id tincidunt purus suscipit. Integer scelerisque odio sed turpis bibendum, a porta ligula sagittis. Fusce ac risus nec ex luctus tempor. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.

// Phasellus tincidunt justo ut odio hendrerit, sed tristique purus posuere. Nam sit amet ligula vitae orci iaculis fermentum. Donec vitae neque vel justo cursus pharetra. Curabitur et ex in urna fermentum imperdiet. Ut id nisl id sem sollicitudin fermentum non a ex. Vivamus porttitor, risus id lacinia aliquet, felis justo facilisis ligula, ut posuere mauris elit sed justo.";
//     let chunks = chunk_text(text, 300, 20).unwrap();
//     let chunk_slices: Vec<&str> = chunks.iter().map(|s| s.as_str()).collect();

//     let embeddings = get_embeddings(chunk_slices).unwrap();
//     let mut db: Vec<VectorDBstore> = Vec::new();
//     println!("{:?}", chunks.len());
//     println!("{:?}", embeddings.len());
// for i in 0..chunks.len() {
//     let item = VectorDBstore::new(i as i32, chunks[i].clone(), embeddings[i].clone());
//     db.push(item);
// }

// for i in 0..db.len() {
//     println!("{:?}\n", db[i]);
// }
// }

#[tokio::main]
async fn main() {
    let db: Arc<RwLock<Vec<VectorDBstore>>> = Arc::new(RwLock::new(Vec::new()));

    let app = Router::new().route("/insert", post(insert)).with_state(db);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
