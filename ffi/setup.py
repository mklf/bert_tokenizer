# -*- coding:utf-8 -*-
import setuptools

setuptools.setup(
    name="bert_tokenizer",
    version="0.1.4",
    author="Li Fang",
    author_email="golifang1234@gmail.com",
    description="A Tokenizer for Bert model",
    packages=setuptools.find_packages(),
    package_data={'':["*.py","*.so","*.dylib","*.dll"]},
    classifiers=[
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 2",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
)