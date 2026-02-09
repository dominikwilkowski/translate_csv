mod csv;
mod translate;

use crate::{csv::CsvParser, translate::Translator};
use std::{fs::File, io::BufReader};

const CSV_PATH: &str = "csv/input.csv";

fn main() {
    let reader = BufReader::new(File::open(CSV_PATH).unwrap());
    let mut csv_file = CsvParser::new(reader);
    // can we rotate emails?
    let translator = Translator::new(Some("hacker@gmail.com".to_string()));

    while let Some(row) = csv_file.next() {
        let phu = translator.translate(&row[8]);
        println!("{phu:?}");
    }
}
