extern crate bert_tokenizer;
use std::error::Error;

use std::mem;
use bert_tokenizer::{
    FullTokenizer,
    get_input_ids
};

fn main()->Result<(),Box<Error>>{
    let tokenizer = FullTokenizer::new("vocab",true)?;
    let s = "UNwant\u{00E9}d,running";
    {
        let tokens = tokenizer.tokenize(s);
        let expected: Vec<String> = ["un", "##want", "##ed", ",","runn","##ing"].iter()
            .map(|s| s.to_string()).collect();
        assert_eq!(tokens,expected);
        println!("{:?} tokenize to {:?}",s,tokens);

        let ids = tokenizer.convert_tokens_to_ids(&tokens);
        println!("{:?}",ids);
    }

    {
        tokenizer.convert_pairs("abc",s,30);

        let input_ids = get_input_ids();
        let vec = unsafe{Vec::from_raw_parts(input_ids,30,30)};

        println!("{:?}",vec);
        let vec2 :Vec<usize>= vec.iter().map(|c| *c as usize).collect();

        let tokens = tokenizer.convert_ids_to_tokens(&vec2);
        println!("{:?}",tokens);
        mem::forget(vec);

    }

    Ok(())
}
