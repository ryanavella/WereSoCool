use weresocool::generation::{NormalizerJson, Op4D, OpCsv1d};

use std::fs::File;

use csv;
use serde_json::from_reader;

fn main() {
    let file = File::open("renders/alex.socool.csv").unwrap();
    let normalizer = File::open("songs/normalizers/alex.socool.normalizer").unwrap();
    let normalizer_json: NormalizerJson = from_reader(&normalizer).unwrap();
    println!("{:?}", normalizer);

    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let op: OpCsv1d = result.unwrap();
        let op: Op4D = op.to_op4d(&normalizer_json);

        println!("{:?}", op);
    }
}

#[test]
fn test_decimal_to_fraction() {
    assert!(true, true);
}
