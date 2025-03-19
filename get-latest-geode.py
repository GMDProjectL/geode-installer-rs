import requests
import os
import zipfile
import json


def get_latest_releases():
    url = "https://api.github.com/repos/geode-sdk/geode/releases"
    response = requests.get(url)
    return response.json()

def get_windows_zip_link():
    latest_releases = get_latest_releases()
    assets = latest_releases[0]["assets"]

    win_zip_release_link = ""

    for asset in assets:
        if "win.zip" in asset["name"]:
            win_zip_release_link = asset["browser_download_url"]
    
    return win_zip_release_link

def download_zip_to_destination(zip_url, destination_dir):
    zip_response = requests.get(zip_url, stream=True)
    zip_file_path = os.path.join(destination_dir, "modloader.zip")
    with open(zip_file_path, 'wb') as f:
        for chunk in zip_response.iter_content(chunk_size=1024): 
            if chunk:
                f.write(chunk)

def main():
    win_zip_release_link = get_windows_zip_link()
    download_zip_to_destination(win_zip_release_link, './resources/geode')


if __name__ == "__main__":
    main()