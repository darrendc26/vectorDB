# vectorDB  

**vectorDB** — an implementation of a vector database written entirely in Rust.  

## 📄 What is this  

vectorDB is a simple, lightweight vector-database implementation in Rust. It allows storage and retrieval of vector data, which can be used for tasks such as nearest-neighbour search, similarity search, and other vector-based queries.  

Use-cases include embedding-based retrieval, semantic search, ML-powered search systems, or any application where vector similarity search is needed.  

## Features  

- Fully written in Rust (no external dependencies)  
- Simple API and easy to integrate into Rust projects  
- Fast vector storage and retrieval  
- Ready-to-use storage format with **Cargo.toml** + **Cargo.lock**  

## Getting Started  

### Prerequisites  

- Rust (stable) installed on your machine  
- `cargo` build tool  

### Installation  

```bash
# Clone the repository  
git clone https://github.com/darrendc26/vectorDB.git  

# Move into the project directory  
cd vectorDB  

# Build the project  
cargo build --release
```
Next we can use the vectorDB via api running on localhost:3000 
