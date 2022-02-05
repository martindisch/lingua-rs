use std::path::Path;

fn main() {
    model_builder::build_frequencies(
        Path::new("models/unigrams.json.zip"),
        Path::new("codegen.rs"),
    );
}
