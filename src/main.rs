use reqwest::Error;
use scraper::{Html, Selector};
        
#[tokio::main]
async fn main() -> Result<(), Error> {
    let body = reqwest::get("https://example.com/quotes").await?.text().await?;
    let fragment = Html::parse_document(&body);
    // Create a selector for the first <p> tag
    let p_selector = Selector::parse("p").unwrap();
    
    // Find the first <p> tag and extract its text
    if let Some(p_tag) = fragment.select(&p_selector).next() {
        let p_text = p_tag.text().collect::<Vec<_>>().join(" ");
        println!("{}", p_text);
    } else {
        println!("No <p> tag found");
    }
    
    Ok(())
}