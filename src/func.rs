use reqwest::{Client,Error};
use scraper::{Html,Selector};
use std::fs::File;
use std::io;

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

//parse latest release
pub async fn parse_tag(url: &str) -> Result<String,Box<dyn std::error::Error>>{
    //save html
    let orig_html = get_html(url).await?;

    //parse html for span tag of f1... class
    let selector = Selector::parse(
        "span.f1.text-bold.d-inline.mr-3 a.Link--primary.Link"
        )
        .unwrap();

    //append all href to vector
    let mut href_vec = Vec::new();
    for element in orig_html.select(&selector){
        if let Some(href) = element.value().attr("href"){
            href_vec.push(href.to_string());
        }
    }
    // debug_fn(&href_vec);

    //obtain latest release tag
    let latest_release = href_vec
        .first()
        .unwrap()
        .split("/")
        .last()
        .unwrap_or("")
        .to_owned();
    // debug_fn(&latest_release);

    Ok(latest_release)
}

//parse specific ver
pub async fn parse_ver(url: String,pattern: &str) -> Result<String,Box<dyn std::error::Error>>{
    //concat to get expanded_assets page
    let new_url = "https://github.com/ankidroid/Anki-Android/releases/expanded_assets/".to_owned()+url.as_str();
    // println!("{new_url}");

    //find all href
    let new_orig_html = get_html(&new_url).await?;
    let new_selector = Selector::parse(
        "div.Box.Box--condensed.mt-3 ul li a[href]"
        )
        .unwrap();

    let clean_urls: Vec<_> = new_orig_html
        .select(&new_selector)
        .filter_map(|element| element.value().attr("href"))
        .collect();
    // debug_fn(&clean_urls);

    //filter vec for parallel.A
    let href_vec_filt: Vec<_> = clean_urls
        .iter()
        .filter(|&n| n.contains(pattern))
        .cloned()
        .collect();
    // debug_fn(&href_vec_filt);

    //final url to download from
    let apk_url = "https://github.com".to_owned()+href_vec_filt[0];

    Ok(apk_url)
}

//download file
type Res<T> = std::result::Result<T,Box<dyn std::error::Error + Send + Sync>>;

pub async fn download_file(url: String,file_name: String) -> Res<()>{
    let response = reqwest::get(&url).await?;
    let mut file = File::create(file_name)?;
    
    //save file
    let mut content = io::Cursor::new(response.bytes().await?);
    io::copy(&mut content,&mut file)?;
    
    Ok(())
}

//create filename from apk_url
pub fn create_filename(arg: String) -> String{
    /*
        split by / -> index [-1] -> convert to &str -> unwrap from Option<&str>
    */
    let filename = arg.split("/").last().unwrap().to_owned();
    return filename;
}

//print resulting html output
pub fn debug_fn<T>(fn_input: &T) where T: std::fmt::Debug{
    println!("{:?}",fn_input);
}