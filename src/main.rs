use std::env;
use select::document::Document;
use select::predicate::Class;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Headline {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read in the first command line argument as the URL to scrape
    let args: Vec<String> = env::args().collect();
    let url = args.get(1).ok_or("Please provide a URL to scrape")?;

    // Send an HTTP GET request to the specified URL
    let resp = reqwest::get(url).await?.text().await?;

    // Parse the HTML response 
    let document = Document::from(resp.as_str());

    // Find the elements on the page that contain the main article headlines
    // Collect 10 elements into a vector
    let mut headlines = Vec::new();
    for headline in document.find(Class("gs-c-promo-heading")).take(10) {
        // Extract the text content of the headline element
        let text = headline.text().trim().to_string();

        // Print the headline to the console
        println!("{}", text);

        // Add the headline to the vector
        headlines.push(Headline { text });
    }

    Ok(())
}
