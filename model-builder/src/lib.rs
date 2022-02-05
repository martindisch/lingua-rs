use phf_codegen::Map;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

pub fn build_frequencies(_models: &Path, output: &Path) {
    let mut file = BufWriter::new(File::create(output).unwrap());

    // TODO: write warning about not modifying the file
    writeln!(
        &mut file,
        "static UNIGRAMS: phf::Map<&str, f64> = \n{};\n",
        Map::new()
            .entry("abc", "0.053872054")
            .entry("bcd", "0.011532")
            .build()
    )
    .unwrap();
}
