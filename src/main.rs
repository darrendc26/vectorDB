mod chunker;
mod embeddings;
use chunker::chunk_text;
use embeddings::get_embeddings;

fn main() {
    let text = "Hello, world! Thidg imhtoekmmmmmmmmmmmmmmmmm['teohpmt toph[itjeeeeeeeeeeeeeeeeeeeee heoph noern ei n ijrn geri oei rgjjjjjjjjjjb rieeeeeeeeeeeeeeeeeeeeeeeeeeeeeejgn rieongrjifns wslf wibn wi4b";
    let chunks = chunk_text(text, 100, 2).unwrap();
    let chunk_slices: Vec<&str> = chunks.iter().map(|s| s.as_str()).collect();

    let embeddings = get_embeddings(chunk_slices).unwrap();
    println!("{:?}", chunks);
    println!("{:?}", embeddings);
}
