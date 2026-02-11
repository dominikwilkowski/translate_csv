mod csv;
mod network;
mod translate;

use crate::{
	csv::{CsvParser, export},
	network::reconnect_and_wait,
	translate::Translator,
};

use rand::RngExt;

use std::{
	fs::{File, OpenOptions},
	io::{BufReader, BufWriter, Write},
};

const CSV_INPUT_PATH: &str = "csv/input.csv";
const CSV_EXPORT_PATH: &str = "csv/output.csv";
const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn random_email(max_len: usize) -> String {
	let mut rng = rand::rng();
	let len = rng.random_range(1..=max_len.min(10));

	let name = (0..len)
		.map(|_| {
			let i = rng.random_range(0..LETTERS.len());
			LETTERS[i] as char
		})
		.collect::<String>();

	format!("{name}@gmail.com")
}

fn main() {
	let reader = BufReader::new(File::open(CSV_INPUT_PATH).unwrap());
	let mut output_file = BufWriter::new(OpenOptions::new().create(true).append(true).open(CSV_EXPORT_PATH).unwrap());

	let csv_file = CsvParser::new(reader);

	// can we rotate emails?
	let mut translator = Translator::new(Some(random_email(5)));
	let mut output = String::new();

	for row in csv_file.skip(35286) {
		loop {
			match translator.translate(&row[8]) {
				Ok(row8) => {
					let mut output_row = Vec::with_capacity(10);
					output_row.extend(row.iter().take(8).cloned());
					output_row.push(row8);
					output_row.push(row[9].clone());

					output.clear();
					export(&vec![output_row], &mut output);
					output_file.write_all(output.as_bytes()).unwrap();
					output_file.flush().unwrap();

					break;
				},
				Err(error) => {
					let _ = reconnect_and_wait("NordVPN NordLynx");
					eprintln!("Translate failed: {error}");
					translator = Translator::new(Some(random_email(6)));
				},
			}
		}
	}
}
