import requests
from bs4 import BeautifulSoup

def create_assets_url(url: str) -> str:
    """Creates assets url to pass into following functions.

    Args:
        url (str): url to github repo releases page

    Returns:
        str: url of assets page for latest release tag
    """
    #obtain latest release tag
    temp_html = BeautifulSoup(requests.get(url).content,"html.parser")
    href = temp_html.find("span",class_="f1 text-bold d-inline mr-3").find("a",class_="Link--primary Link").get("href")
    tag = href.split("/")[-1]

    #create full url for expanded assets page
    expanded_assets = "https://github.com/ankidroid/Anki-Android/releases/expanded_assets/"
    full_url = f"{expanded_assets}{tag}"
    
    return full_url

def all_asset_urls(url: str) -> list:
    """Return list of all versions available for download.

    Args:
        url (str): url to assets page for latest release tag

    Returns:
        list: All versions available for download as list of strings.
    """
    #parse expanded assets page
    temp_html = BeautifulSoup(requests.get(url).content,"html.parser")
    apk_link = temp_html.find("div",class_="Box Box--condensed mt-3").find("ul").find_all("li")
    
    #store all apk urls in list
    cleanURLs = []
    for i in range(len(apk_link)):
        cleanURLs.append(apk_link[i].find("a",href=True)["href"])
        
    return cleanURLs

def download_apk(url_list: list,selection: str):
    """Downloads apk based on passed args.

    Args:
        url_list (list): List of all versions available for download.
        selection (str): Simple filter to select single apk to download. Will always download 1st entry (index 0) if > 1 remains after selection.
    """
    #filter by selection criteria (will only return single string)
    single_apk = list(filter(lambda x: selection in x,url_list))[0]
    
    #create filename & url
    filename = single_apk.split("/")[-1]
    final_url = f"https://github.com{single_apk}"
    
    #save in current dir
    try:
        res = requests.get(final_url)
        with open(filename,"wb") as file:
            file.write(res.content)
        print("Success")
    except:
        print(f"Unable to save {filename} from {final_url}.")