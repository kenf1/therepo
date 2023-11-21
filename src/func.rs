use reqwest::{Client,Error};
use scraper::Html;

//return html from input url
pub async fn get_html(url: &str) -> Result<Html,Error>{
    let body = Client::new()
        .get(url)
        .send()
        .await?
        .text()
        .await?;
    let raw_html = Html::parse_document(&body); //convert body to html object (parse prep)
    
    Ok(raw_html)
}

//print resulting html output
pub fn debug_fn<T>(fn_input: &T) where T: std::fmt::Debug{
    println!("{:?}",fn_input);
}