# -*- coding: utf-8 -*-
# @Time : 2021/4/23 17:08
# @Author : yuggu
# @Site : 
# @File : wevdirverUsage.py
# @Software: PyCharm

from selenium.webdriver import Chrome
from selenium.webdriver.common.keys import Keys
import time
import requests


def board_crawl(url, web):
    pass



web = Chrome()

headers = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.85 Safari/537.36"}

test_url_board = "https://huaban.com/pins/3899008225/"
init_url = "https://huaban.com/discovery/beauty/"

web.get(test_url_board)
time.sleep(1)



# xpath can only help us get html tag but can not get attribute
pic_li = web.find_element_by_xpath("//*[@id='baidu_image_holder']/img")

print(pic_li.get_attribute("src"))
file_name = pic_li.get_attribute("alt") + '.webp'
with open(file_name, "wb",) as f:
    rep =requests.get(pic_li.get_attribute("src"), headers=headers)
    f.write(rep.content)
    rep.close()
    f.close()

web.close()
