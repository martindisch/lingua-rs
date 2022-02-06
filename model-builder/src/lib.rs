use common::{fraction::Fraction, language::Language, ngram::Ngram};
use phf_codegen::Map;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
    path::Path,
};
use zip::ZipArchive;

pub fn build_frequencies(models: &Path, output: &Path) {
    let mut file = BufWriter::new(File::create(output).unwrap());

    for n in 1..=5 {
        let json = load_json(n).unwrap();
        let frequencies = from_json(&json);

        let mut builder = Map::new();
        for (key, value) in &frequencies {
            builder.entry(&key.value, &format!("{}_f64", value));
        }

        // TODO: write warning about not modifying the file
        writeln!(
            &mut file,
            "static FREQUENCIES_{}: phf::Map<&str, f64> = \n{};\n",
            n,
            builder.build()
        )
        .unwrap();
    }
}

fn load_json(ngram_length: usize) -> io::Result<String> {
    let ngram_name = Ngram::find_ngram_name_by_length(ngram_length);
    let file_path = format!("models/{}s.json.zip", ngram_name);
    let file_reader = BufReader::new(File::open(file_path).unwrap());

    let mut archive = ZipArchive::new(file_reader).unwrap();
    let mut json_file = archive.by_index(0).unwrap();

    let mut json = String::new();
    json_file.read_to_string(&mut json)?;

    Ok(json)
}

fn from_json(json: &str) -> HashMap<Ngram, f64> {
    let json_language_model = serde_json::from_str::<JsonLanguageModel>(json).unwrap();
    let mut json_relative_frequencies = HashMap::new();

    for (fraction, ngrams) in json_language_model.ngrams {
        let floating_point_value = fraction.to_f64();
        for ngram in ngrams.split(' ') {
            json_relative_frequencies.insert(Ngram::new(ngram), floating_point_value);
        }
    }

    json_relative_frequencies
}

#[derive(Deserialize)]
struct JsonLanguageModel {
    language: Language,
    ngrams: BTreeMap<Fraction, String>,
}
