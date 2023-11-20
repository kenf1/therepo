from func import *
import os

def main():
    releases = "https://github.com/ankidroid/Anki-Android/releases"

    asset_url = create_assets_url(releases)
    all_apk_url = all_asset_urls(asset_url)

    meta_data = create_metadata(all_apk_url,"parallel.A")

    #check if version already exists in Storage folder
        #automatically downloads if Storage folder is empty
    if len(os.listdir("./Storage")) == 0:
        download_apk(meta_data)
    else:
        if meta_data.filename == os.listdir("./Storage")[0]:
            print("Latest version already exists in Storage folder.")
        else:
            download_apk(meta_data)

if __name__ == "__main__":
    main()