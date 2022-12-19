# -*- coding:utf-8 -*-
from bert_tokenizer import FullTokenizer
import json
for text in [
    "笔记本",
    "不存在",
    "地理槽位多轮"
]:
    by = ",".join(map(str, text.encode('utf8')))
    print(by)
tokenizer = FullTokenizer()
print(tokenizer.convert_pairs("你好",u"UNwant\u00E9d,running",20, 1))
print(tokenizer.convert_pairs(u"你好",u"UNwant\u00E9d,running",20, 1))

print(tokenizer.convert_pairs(u"你好帅",u"帅哥帅哥",0, 1))
print(tokenizer.convert_pairs(u"你好帅",u"帅哥帅哥",0, 1))