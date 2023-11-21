use scraper::Selector;
use std::fs::File;
use std::io::Write;

mod func;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let url = "https://github.com/ankidroid/Anki-Android/releases";

    //save html
    let orig_html = func::get_html(url).await?;

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
    // func::debug_fn(&href_vec);

    //obtain latest release tag
    let latest_release = href_vec[0].split("/").last().unwrap_or("");
    // func::debug_fn(&latest_release);

    //concat to get expanded_assets page
    let new_url = "https://github.com/ankidroid/Anki-Android/releases/expanded_assets/".to_owned()+latest_release;
    // println!("{new_url}");

    //find all href
    let new_orig_html = func::get_html(&new_url).await?;
    let new_selector = Selector::parse(
        "div.Box.Box--condensed.mt-3 ul li a[href]"
        )
        .unwrap();

    let clean_urls: Vec<_> = new_orig_html
        .select(&new_selector)
        .filter_map(|element| element.value().attr("href"))
        .collect();
    // func::debug_fn(&clean_urls);

    //filter vec for parallel.A
    let href_vec_filt: Vec<_> = clean_urls
        .iter()
        .filter(|&n| n.contains("parallel.A"))
        .cloned()
        .collect();
    // func::debug_fn(&href_vec_filt);

    //final url to download from
    let apk_url = "https://github.com".to_owned()+href_vec_filt[0];
    func::debug_fn(&apk_url);

    //download file

    Ok(())
}