use serde::Deserialize;
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
struct ApiResponseData {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    #[serde(rename = "responseData")]
    response_data: ApiResponseData,
    #[serde(rename = "responseStatus")]
    response_status: u32,
}

pub struct Translator {
    client: reqwest::blocking::Client,
    email: Option<String>,
    delay: Duration,
}

impl Translator {
    pub fn new(email: Option<String>) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            email,
            delay: Duration::from_millis(1100),
        }
    }

    pub fn translate(&self, text: &str) -> Result<String, String> {
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        let mut params = vec![("q", text), ("langpair", "tet|en")];
        let email_val;
        if let Some(ref email) = self.email {
            email_val = email.clone();
            params.push(("de", &email_val));
        }

        let response = self
            .client
            .get("https://api.mymemory.translated.net/get")
            .query(&params)
            .send()
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API returned status {}", response.status()));
        }

        let body: ApiResponse = response
            .json()
            .map_err(|e| format!("Failed to parse API response: {}", e))?;

        if body.response_status != 200 {
            return Err(format!("API response status: {}", body.response_status));
        }

        Ok(body.response_data.translated_text)
    }

    pub fn rate_limit_pause(&self) {
        thread::sleep(self.delay);
    }
}

pub fn translate_column(
    rows: &mut [Vec<String>],
    col_index: usize,
    translator: &Translator,
) -> usize {
    let total = rows.len();
    let mut error_count = 0;

    for (i, row) in rows.iter_mut().enumerate() {
        // Skip header row
        if i == 0 {
            continue;
        }

        if col_index >= row.len() {
            continue;
        }

        let original = &row[col_index];
        if original.trim().is_empty() {
            continue;
        }

        eprint!("\rTranslating row {}/{}...", i, total - 1);

        match translator.translate(original) {
            Ok(translated) => {
                row[col_index] = translated;
            }
            Err(e) => {
                eprintln!("\nWarning: Failed to translate row {}: {}", i, e);
                error_count += 1;
            }
        }

        // Rate limit between API calls
        if i < total - 1 {
            translator.rate_limit_pause();
        }
    }

    eprintln!(
        "\rTranslation complete: {}/{} rows translated successfully.",
        total - 1 - error_count,
        total - 1
    );

    error_count
}
