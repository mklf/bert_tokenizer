extern crate unicode_normalization;
extern crate unicode_categories;
use tokenization::unicode_categories::UnicodeCategories;

use tokenization::unicode_normalization::UnicodeNormalization;



pub struct BasicTokenizer {
    pub do_lower_case: bool
}

const WHITESPACE_CHARACTER: char = b' ' as char;

impl BasicTokenizer {
    pub fn new(do_lower_case: bool) -> BasicTokenizer {
        BasicTokenizer {
            do_lower_case: do_lower_case
        }
    }
    fn _clean_text<T: AsRef<str>>(&self, text: T) -> Vec<char> {
        let text = text.as_ref();
        let mut output = Vec::new();
        for character in text.chars() {
            let char_value = character as u32;
            if char_value == 0 || char_value == 0xffd || character.is_control() {
                continue;
            }
            if character.is_whitespace() {
                output.push(WHITESPACE_CHARACTER);
            } else {
                output.push(character);
            }
        }
        output
    }

    fn _is_chinese_char(cp: u32) -> bool {
        return  (cp >= 0x4E00 && cp <= 0x9FFF) ||
            (cp >= 0x3400 && cp <= 0x4DBF) ||
            (cp >= 0x20000 && cp <= 0x2A6DF) ||
            (cp >= 0x2A700 && cp <= 0x2B73F) ||
            (cp >= 0x2B740 && cp <= 0x2B81F) ||
            (cp >= 0x2B820 && cp <= 0x2CEAF) ||
            (cp >= 0xF900 && cp <= 0xFAFF) ||
            (cp >= 0x2F800 && cp <= 0x2FA1F)
    }

    fn _tokenize_chinese_chars(&self, text: Vec<char>) -> Vec<char> {
        let mut output = Vec::new();
        for character in text {
            let cp = character as u32;
            if BasicTokenizer::_is_chinese_char(cp){
                output.push(WHITESPACE_CHARACTER);
                output.push(character);
                output.push(WHITESPACE_CHARACTER);
            }else {
                output.push(character);
            }
        }
        output
    }

    fn _run_strip_accents<T:AsRef<str>>(text:T)->String{
        let text = text.as_ref();
        text.nfd().filter(|c|!c.is_mark_nonspacing()).collect()
    }

    fn _is_punctuation(character:char) ->bool{
        let cp = character as u32;
        if cp>=33 && cp <=47 || cp >=58 && cp<=64 || cp>=91 && cp<=96 || cp>=123 && cp<=126{
           return true
        }
        character.is_punctuation()
    }


    pub fn tokenize<T>(&self, text: T)->Vec<String> where T: AsRef<str> {
        let text = text.as_ref();

        let text = self._clean_text(text);

        let text = self._tokenize_chinese_chars(text);

        let process_str :String = text.iter().collect();

        let tokens:Vec<String> = process_str.split_whitespace()
            .map(|s| s.to_lowercase())
            .map(BasicTokenizer::_run_strip_accents)
            .flat_map(|c| c.split(BasicTokenizer::_is_punctuation).
                                    map(|p|p.to_string()).collect::<Vec<String>>())
            .filter(|s|s.len()>0).collect();

        println!("{:?}", tokens);
        tokens
    }
}

struct

pub struct FullTokenizer {
    basic_tokenizer : BasicTokenizer
}