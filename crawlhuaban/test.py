# -*- coding: utf-8 -*-
# @Time : 2021/4/23 10:33
# @Author : yuggu
# @Site : 
# @File : test.py
# @Software: PyCharm

import requests



url = "https://www.pearvideo.com/video_1727462"
contID = url.split("_")[1]

requestsurl = f"https://www.pearvideo.com/videoStatus.jsp?contId={contID}&mrd=0.19113760262044055"

headers = {
"User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.85 Safari/537.36",
"Referer": url
}

resp = requests.get(requestsurl, headers = headers)
dic = resp.json()

srcurl = dic['videoInfo']['videos']['srcUrl']

systemTime = dic['systemTime']

srcurl = srcurl.replace(systemTime, f"cont-{contID}")

with open("a.mp4", "wb") as f:
    f.write(requests.get(srcurl).content)
    f.close()