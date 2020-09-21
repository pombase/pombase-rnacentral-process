use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct RfamAnnotation {
#[serde(rename = "URS-Id")]
    pub urs_identifier: String,
#[serde(rename = "Rfam-Model-Id")]
    pub rfam_model_id: String,
#[serde(rename = "Score")]
    pub score: f32,
#[serde(rename = "E-value")]
    pub e_value: f32,
#[serde(rename = "Sequence-Start")]
    pub sequence_start: u32,
#[serde(rename = "Sequence-Stop")]
    pub sequence_stop: u32,
#[serde(rename = "Model-Start")]
    pub model_start: u32,
#[serde(rename = "Model-Stop")]
    pub model_stop: u32,
#[serde(rename = "Rfam-Model-Description")]
    pub rfam_model_description: String,
}

// Parse the URS identifers from identifiers_filename, then parse the Rfam
// annotations from rfam_annotations_filename.  Return a Vec of the annotations
// where the URS ID is in the identifiers file.
pub fn parse(identifiers_filename: &str, rfam_annotations_filename: &str)
             -> Result<HashMap<String, Vec<RfamAnnotation>>, Box<dyn Error>>
{
    let identifiers_file = File::open(identifiers_filename)?;
    let identifiers_reader = BufReader::new(identifiers_file);

    let mut identifiers: HashSet<String> = HashSet::new();

    let mut identifiers_csv_reader =
        csv::ReaderBuilder::new().delimiter(b'\t').from_reader(identifiers_reader);
    for result in identifiers_csv_reader.records() {
        let record = result?;
        identifiers.insert(record.get(0).unwrap().to_owned());
    }

    let annotations_file = File::open(rfam_annotations_filename)?;
    let annotations_reader = BufReader::new(annotations_file);

    let mut annotations_csv_reader =
        csv::ReaderBuilder::new().delimiter(b'\t').has_headers(false)
        .from_reader(annotations_reader);

    let mut results: HashMap<String, Vec<RfamAnnotation>> = HashMap::new();

    for annotation_result in annotations_csv_reader.deserialize() {
        let record: RfamAnnotation = annotation_result?;

        if identifiers.contains(&record.urs_identifier) {
            results.entry(record.urs_identifier.clone())
                .or_insert_with(Vec::new).push(record);
        }
    }

    Ok(results)
}



#[test]
fn test_parse() {
    let mut base = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.push("tests/data");

    let mut rfam_annotations_filename = base.clone();
    rfam_annotations_filename.push("rfam-annotations.tsv");
    let mut identifier_filename = base.clone();
    identifier_filename.push("pombase-identifiers.tsv");

    let res = parse(identifier_filename.to_str().unwrap(),
                    rfam_annotations_filename.to_str().unwrap()).unwrap();

    assert_eq![res.len(), 12];

    let first_res = res.get("URS000003EB75").unwrap().get(0).unwrap();
    assert_eq![first_res.urs_identifier, "URS000003EB75"];
    assert_eq![first_res.rfam_model_id, "RF00005"];
    assert_eq![first_res.rfam_model_description, "tRNA"];
}
