use serde::Deserialize;

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
}

impl Translator {
	pub fn new(email: Option<String>) -> Self {
		Self {
			client: reqwest::blocking::Client::new(),
			email,
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

		let body: ApiResponse = response.json().map_err(|e| format!("Failed to parse API response: {}", e))?;

		if body.response_status != 200 {
			return Err(format!("API response status: {}", body.response_status));
		}

		Ok(body.response_data.translated_text)
	}
}
