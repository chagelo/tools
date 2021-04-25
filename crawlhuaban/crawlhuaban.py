# -*- coding: utf-8 -*-
# @Time : 2021/4/22 12:08
# @Author : yuggu
# @Site : 
# @File : crawlhuaban.py
# @Software: PyCharm

import time
import requests
import json
import re

headers = {
        "User-Agent":"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.85 Safari/537.36"
}

url_re = r''

def crawlonepic(key,pin_id):
    pic_url = 'https://hbimg.huabanimg.com/' + key
    resp = requests.get(url, headers = headers)
    with open("D:/crawl_pic/" + ".".join(pin_id, "jpg"), "wb") as f:
        f.write(resp.content)
        f.close()
    resp.close()

def crawlpartofoneboards(board_url):
    cnt = 0
    while  cnt < 100:
        resp = requests.get(board_url, headers = headers)
        pins = json.loads(resp.text)
        print(json)
        pass

def parasehtml(url):
        url = "https://huaban.com/discovery/beauty/boards/"
        resptext = requests.get(url, headers = headers).text
        pattern = re.compile('app.page\["boards"\] = \[(.*?)\]\;app.page', re.S)
        boardsinfo = str(re.findall(pattern, resptext));
        print(boardsinfo)
        print(json.loads(boardsinfo))



if __name__ == '__main__':
    parasehtml("")
    pass
    # url = 'https://huaban.com/boards/17431724/?knv52ttf&max=3631181031&limit=20&wfl=1'
    # headers['Referer'] = 'https://huaban.com/boards/17431724/'
    # resp = requests.get(url, headers=headers)
    # print(resp.text)
    # pins = json.loads(resp.text)['pins']
    # print(pins)
