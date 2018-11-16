extern crate bert_tokenizer;
use std::error::Error;
use bert_tokenizer::{
    FullTokenizer
};

fn main()->Result<(),Box<Error>>{
    let tokenizer = FullTokenizer::new("vocab",true)?;
    {
        let s = "UNwant\u{00E9}d,running";
        let tokens = tokenizer.tokenize(s);
        let expected: Vec<String> = ["un", "##want", "##ed", ",","runn","##ing"].iter()
            .map(|s| s.to_string()).collect();
        assert_eq!(tokens,expected);
        println!("{:?} tokenize to {:?}",s,tokens);
    }
    Ok(())
}
