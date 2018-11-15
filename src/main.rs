extern crate bert_tokenizer;
use bert_tokenizer::{
    FullTokenizer,
    BasicTokenizer
};

fn main() {
    let basic_tokenizer = BasicTokenizer::new(true);
    basic_tokenizer.tokenize("abc def你好mn");
    basic_tokenizer.tokenize("");
}
