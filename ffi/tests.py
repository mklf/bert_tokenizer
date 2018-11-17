# -*- coding:utf-8 -*-
from bert_tokenizer import FullTokenizer

tokenizer = FullTokenizer("vocab")
print(tokenizer.convert_pairs("你好",u"UNwant\u00E9d,running",20))
print(tokenizer.convert_pairs(u"你好",u"UNwant\u00E9d,running",20))