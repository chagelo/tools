# -*- coding: utf-8 -*-
# @Time : 2021/4/23 13:23
# @Author : yuggu
# @Site : 
# @File : crawl_2.py
# @Software: PyCharm

import requests
import re
import json

content = requests.get('https://huaban.com/discovery/beauty/').text

startIndex = re.search('\[\"pins\"\]\ =\ ', content).span()[1]
endIndex = re.search('}}];', content[startIndex:]).span()[1] + startIndex-1

jsonStr = content[startIndex:endIndex]
jsonObj = json.loads(jsonStr)

print(jsonObj)

for item in jsonObj:
    url = 'https://hbimg.huabanimg.com/' + item['file']['key']
    image = requests.get(url).content
    with open('D:/crawl_pic/' + item['file']['key']+'.jpg', 'wb') as file:
        file.write(image)