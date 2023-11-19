import requests
from bs4 import BeautifulSoup

#Create download link

url = "https://github.com/ankidroid/Anki-Android/releases"
html_res = BeautifulSoup(requests.get(url).content,"html.parser")
href = html_res.find("span",class_="f1 text-bold d-inline mr-3").find("a",class_="Link--primary Link").get("href")
# print(href)

tag = href.split("/")[-1]
# print(tag)

expanded_assets = "https://github.com/ankidroid/Anki-Android/releases/expanded_assets/"

full_url = f"{expanded_assets}{tag}"
# print(full_url)

apk_res = BeautifulSoup(requests.get(full_url).content,"html.parser")
apk_link = apk_res.find("div",class_="Box Box--condensed mt-3").find("ul").find_all("li")
# print(apk_link)

cleanURLs = []
for i in range(len(apk_link)):
    cleanURLs.append(apk_link[i].find("a",href=True)["href"])

# print(cleanURLs[0])

parallelA = list(filter(lambda x: "parallel.A" in x,cleanURLs))[0]
print(parallelA)

#Download file

final_url = f"https://github.com{parallelA}"
# print(final_url)

res = requests.get(final_url)
print(res)

filename = parallelA.split("/")[-1]

with open(filename,'wb') as file:
    file.write(res.content)