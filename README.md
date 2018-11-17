# bert_tokenizer

*bert_tokenizer* 是 google bert 模型tokenizer的rust实现。同时提供了对python/c/c++的支持

python 版可以通过
`
    pip install bert_tokenizer
`
安装，win/linux/macosx均提供支持

对c和c++应用，引用`ffi/bert_tokenizer.h`头文件，并链接
`ffi/bert_tokenizer/libbert_tokenizer.{so,a,dylib,dll}`.

