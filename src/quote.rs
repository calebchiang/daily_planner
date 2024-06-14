use reqwest;
use serde::Deserialize;
use std::error::Error;
use std::env;
use dotenv::dotenv; // Ensure you've imported dotenv

#[derive(Deserialize, Debug)]
struct ApiResponse {
    contents: QuoteContents
}

#[derive(Deserialize, Debug)]
struct QuoteContents {
    quotes: Vec<Quote>
}

#[derive(Deserialize, Debug)]
struct Quote {
    quote: String,
    author: String,
}

pub fn fetch_quote() -> Result<String, Box<dyn Error>> {
    dotenv().ok(); // Load environment variables from .env file

    let api_key = env::var("QUOTE_API_KEY")?; // Retrieve API key from environment variables
    let url = "https://quotes.rest/qod.json?category=inspire&language=en";
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url)
        .header("X-TheySaidSo-Api-Secret", api_key) // Include the API key in the request header
        .send()?;

    if resp.status().is_success() {
        let response: ApiResponse = resp.json()?;
        if let Some(quote) = response.contents.quotes.get(0) {
            return Ok(format!("\"{}\" - {}", quote.quote, quote.author));
        } else {
            return Err("No quote found".into());
        }
    } else {
        return Err(format!("Failed to fetch quote: {}", resp.status()).into());
    }
}
