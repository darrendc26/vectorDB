use fastembed::TextEmbedding;

pub fn get_embeddings(text: Vec<&str>) -> Result<Vec<Vec<f32>>, String> {
    let mut model = TextEmbedding::try_new(Default::default()).map_err(|e| format!("{:?}", e))?;
    let embeddings = model
        .embed(text, Some(1000))
        .map_err(|e| format!("{:?}", e))?;

    Ok(embeddings)
}
