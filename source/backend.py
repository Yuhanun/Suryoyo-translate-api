import re
import json
import string
import typing
import functools

import bs4
import requests

def get_url(term: str) -> str:
    return f"https://www.syriacdictionary.net/index.cgi?langsel=en&word={term}"

def fetch_page_for(term: str) -> typing.Tuple[bool, str]:
    return fetch_page(get_url(term))

def fetch_page(url: str) -> typing.Tuple[bool, str]:
    res = requests.get(url)
    return res.status_code == 200, res.content

def into_bs4(content: str) -> bs4.BeautifulSoup:
    return bs4.BeautifulSoup(content, "lxml")

def tag_or_string(either: typing.Union[bs4.element.Tag, bs4.element.NavigableString]):
    if isinstance(either, bs4.element.Tag):
        return either.string
    return either.string

def sanitize_suryoyo(suryoyo: str) -> str:
    for each in string.ascii_letters:
        suryoyo = suryoyo.replace(each, "")
    return suryoyo.strip()

@functools.lru_cache(maxsize=None)
def get_translations_for(term: str):
    """
    Output will be:
    {
        "error": null | str,
        "results": {
            "number": { "{english}": ["{suryoyo}", "{suryoyo}"] },
            "number": { "{english}": ["{suryoyo}"] },
        }
    }
    """
    output = {"error": None, "results": {}}
    fetched = fetch_page_for(term)[1]
    bs4 = into_bs4(fetched)
    messages = bs4.find_all("div", {"class": "message"})
    if messages:
        return json.dumps({"error": f"No entry found for {term}.", "results": {}})
    result_box = bs4.find("div", id="content")
    records = result_box.find_all("div", id="recordContainer")
    for record in records:
        num = record.find("div", id="recordnr").find("b").string
        table = record.find("table", {"class": "bbttaabbllee"})
        current_num = {}
        english_last = None
        for rule in table.find_all("tr"):
            english = rule.find("td", {"id": re.compile("translation*")})
            suryoyo = rule.find("td", {"class": "sy"})
            if not suryoyo:
                continue
            if not suryoyo.string:
                continue
            
            english_content = "".join(tag_or_string(each) for each in english.contents).strip()
            suryoyo_contents = sanitize_suryoyo("".join(suryoyo.contents))
            if english_content is not None:
                english_last = english_content
            if english_last not in current_num:
                current_num[english_last] = []

            current_num[english_last].append(suryoyo_contents)
        output["results"][num] = current_num

    return json.dumps(output)
