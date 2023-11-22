use std::path::Path;

mod func;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let url = "https://github.com/ankidroid/Anki-Android/releases";

    //get specific version url
    let latest_release = func::parse_tag(url).await?;
    let apk_url = func::parse_ver(latest_release,"arm64").await?;
    // func::debug_fn(&apk_url);

    /*
        create file name from apk_url
        cloned to allow passing into 2 functions
    */
    let parent_path = "./Storage";
    let file_name = format!("{}/{}",parent_path,func::create_filename(apk_url.clone()));
    // func::debug_fn(&file_name);

    /*
        skip download if latest version exists locally
    */
    match Path::new(&file_name).exists(){
        true => {
            println!("Latest version already exists in `{parent_path}`");
        },
        false => {
            func::download_file(apk_url.to_string(),file_name.to_string()).await.unwrap();
            println!("Success!");
        }
    }

    Ok(())
}