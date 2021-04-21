# -*- coding: utf-8 -*-
# @Time : 2021/4/21 14:18
# @Author : yuggu
# @Site : 
# @File : md2html.py
# @Software: PyCharm


import markdown
import os.path
import sys
from bs4 import BeautifulSoup


class Markdown2Html:

    def __init__(self):
        self.headtag = '<head><meta charset="utf-8" />' \
                       '<script type="text/javascript" src="http://cdn.mathjax.org/mathjax/latest/MathJax.js?config=TeX-AMS-MML_HTMLorMML"></script>'\
                        '<script type="text/x-mathjax-config">'\
                            'MathJax.Hub.Config({ tex2jax: {inlineMath: [[\'$\', \'$\']]}, messageStyle: "none" });'\
                       '</script>'\
                       '</head>'

    def setStyle(self, cssfile):
        '''
        设置样式表文件
        '''
        with open(cssfile, 'r') as f:
            css = f.read()
            self.headtag = self.headtag[:-7] + f'<style  type="text/css">{css}</style>' + self.headtag[-7:]

    def convert(self, rfile, wfile=None, prettify=False)->None:
        if not os.path.isfile(rfile):
            print("%s is not a file! Check the name try again!" % rfile)
            return
        if not os.path.exists(rfile):
            print("%s not found! check the name try again!" % rfile)
            return
        if not rfile.endswith('.md'):
            print("%s type error!" % rfile)
            return

        with open(rfile, "r", encoding='utf-8') as f:
            markdowntext = f.read()
        f.close()

        if wfile is None:
            wfile = os.path.splitext(rfile)[0] + '.html'
        elif wfile.endswith('.html'):
            wfile + '.html'

        rawhtml = self.headtag + markdown.markdown(markdowntext, output='html5', extensions=['extra'])

        if prettify:
            rawhtml = BeautifulSoup(rawhtml, 'html5lib').prettify()

        with open(wfile, 'w', encoding='utf-8') as f:
            f.write(rawhtml)

        return

if __name__=='__main__':
    m2h = Markdown2Html()
    rfile, wfile = sys.argv[1], sys.argv[2]
    if len(sys.argv) == 4:
        cssstyle = sys.argv[3]
        m2h.setStyle(cssstyle)
    m2h.convert(rfile, wfile)
    print("convert successfully")
