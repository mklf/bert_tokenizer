# -*- coding:utf-8 -*-

from distutils.core import setup, Extension

extension_module = Extension(
    'bert_tokenzier.extension',
     sources=["bert_tokenizer/dummy.c"],
)

setup(
    name = 'bert_tokenizer',
    version = '1.0',
    description = 'tokenizer mudule for bert.',
    ext_modules = [extension_module],
    packages=["bert_tokenizer"],
)
