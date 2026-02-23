mod csv;
mod translate;

use crate::{
	csv::{CsvParser, export},
	translate::Translator,
};

use rand::RngExt;

use std::{
	fs::{File, OpenOptions},
	io::{BufReader, BufWriter, Write},
	thread::sleep,
	time::Duration,
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
	Working,
	Waiting,
}

fn main() {
	let reader = BufReader::new(File::open(CSV_INPUT_PATH).unwrap());
	let mut output_file = BufWriter::new(OpenOptions::new().create(true).append(true).open(CSV_EXPORT_PATH).unwrap());

	let csv_file = CsvParser::new(reader);

	// can we rotate emails?
	let mut translator = Translator::new(Some(random_email(5)));
	let mut output = String::new();
	let mut state = State::Working;

	for cell in csv_file.skip(0) {
		loop {
			match translator.translate(&cell[3]) {
				Ok(desc) => {
					if state == State::Waiting {
						state = State::Working;
						println!("\x1B[42m\x1B[37m Translation working again!\x1B[0m");
					}
					let mut output_row = Vec::with_capacity(11);
					output_row.extend(cell.iter().take(3).cloned());
					output_row.push(desc);
					output_row.extend(cell.iter().skip(4).take(7).cloned());

					output.clear();
					export(&vec![output_row], &mut output);
					output_file.write_all(output.as_bytes()).unwrap();
					output_file.flush().unwrap();

					break;
				},
				Err(error) => {
					if state == State::Working {
						state = State::Waiting;
						println!("\x1B[41m\x1B[37m Translation failed with error: {error} • Retrying!\x1B[0m");
					}
					sleep(Duration::from_millis(500));
					translator = Translator::new(Some(random_email(6)));
				},
			}
		}
	}
}
