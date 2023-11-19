from func import *

def main():
    releases = "https://github.com/ankidroid/Anki-Android/releases"

    asset_url = create_assets_url(releases)
    all_apk_url = all_asset_urls(asset_url)

    download_apk(all_apk_url,"parallel.A")

if __name__ == "__main__":
    main()