use tiktoken_rs::get_bpe_from_model;

pub fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Result<Vec<String>, String> {
    let bpe = get_bpe_from_model("text-embedding-3-small")
        .map_err(|e| format!("Tokenizer error: {:?}", e))?;
    let tokens = bpe.encode_with_special_tokens(text);

    let mut chunks = Vec::new();
    let mut start = 0;

    while start < tokens.len() {
        let end = (start + chunk_size).min(tokens.len());
        let chunk = tokens[start..end].to_vec();

        let chunk_text = bpe
            .decode(chunk)
            .map_err(|e| format!("Decode error: {:?}", e))?;
        chunks.push(chunk_text);

        if end == tokens.len() {
            break;
        }
        start += chunk_size.saturating_sub(overlap);
    }

    Ok(chunks)
}
