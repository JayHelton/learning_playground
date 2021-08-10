#! /usr/bin/python3
import requests
from bs4 import BeautifulSoup
import hashlib


def get_challenge(body):
    soup = BeautifulSoup(body, "html.parser")
    header = soup.select("h3")[0].text.strip()
    print("header ", header)
    return header


def post(session, hash):
    print("sending hash ", hash)
    r = session.post("http://139.59.166.56:32107/", data={"hash": hash})
    print(r.text)


def main():
    session = requests.session()
    r = session.get("http://139.59.166.56:32107/")
    header = get_challenge(r.text)
    hash = hashlib.md5(header.encode("utf-8")).hexdigest()
    post(session, hash)


main()
