use indexmap::IndexMap;
use std::error::Error;
use unicode_categories::UnicodeCategories;
use unicode_normalization::UnicodeNormalization;

pub struct BasicTokenizer {
    pub do_lower_case: bool,
}

const WHITESPACE_CHARACTER: char = b' ' as char;

fn _is_punctuation(character: char) -> bool {
    let cp = character as u32;
    if cp >= 33 && cp <= 47
        || cp >= 58 && cp <= 64
        || cp >= 91 && cp <= 96
        || cp >= 123 && cp <= 126
    {
        return true;
    }
    character.is_punctuation()
}

fn _is_control(character: char) -> bool {
    if character == '\t' || character == '\n' || character == '\r' {
        return false;
    }
    return character.is_control();
}

impl BasicTokenizer {
    pub fn new(do_lower_case: bool) -> BasicTokenizer {
        BasicTokenizer { do_lower_case }
    }
    fn _clean_text<T: AsRef<str>>(&self, text: T) -> Vec<char> {
        let text = text.as_ref();
        let mut output = Vec::new();
        for character in text.chars() {
            let char_value = character as u32;
            if char_value == 0 || char_value == 0xffd || _is_control(character) {
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
        return (cp >= 0x4E00 && cp <= 0x9FFF)
            || (cp >= 0x3400 && cp <= 0x4DBF)
            || (cp >= 0x20000 && cp <= 0x2A6DF)
            || (cp >= 0x2A700 && cp <= 0x2B73F)
            || (cp >= 0x2B740 && cp <= 0x2B81F)
            || (cp >= 0x2B820 && cp <= 0x2CEAF)
            || (cp >= 0xF900 && cp <= 0xFAFF)
            || (cp >= 0x2F800 && cp <= 0x2FA1F);
    }

    fn _tokenize_chinese_chars(&self, text: Vec<char>) -> Vec<char> {
        let mut output = Vec::new();
        for character in text {
            let cp = character as u32;
            if BasicTokenizer::_is_chinese_char(cp) {
                output.push(WHITESPACE_CHARACTER);
                output.push(character);
                output.push(WHITESPACE_CHARACTER);
            } else {
                output.push(character);
            }
        }
        output
    }

    fn _run_strip_accents<T: AsRef<str>>(text: T) -> String {
        let text = text.as_ref();
        text.nfd().filter(|c| !c.is_mark_nonspacing()).collect()
    }

    fn _run_split_on_punc<T: AsRef<str>>(text: T) -> Vec<String> {
        let chars: Vec<char> = text.as_ref().chars().collect();
        let mut i = 0;
        let mut output: Vec<Vec<char>> = Vec::new();
        let mut start_new_word = true;
        while i < chars.len() {
            let char_ = chars[i];
            if _is_punctuation(char_) {
                output.push(vec![char_]);
                start_new_word = true;
            } else {
                if start_new_word {
                    output.push(vec![char_]);
                } else {
                    output.last_mut().unwrap().push(char_);
                }
                start_new_word = false;
            }
            i += 1;
        }
        output.iter().map(|v| v.iter().collect()).collect()
    }

    pub fn tokenize<T>(&self, text: T) -> Vec<String>
    where
        T: AsRef<str>,
    {
        let text = text.as_ref();
        let text = self._clean_text(text);

        let text = self._tokenize_chinese_chars(text);

        let process_str: String = text.iter().collect();

        let tokens: Vec<String> = process_str
            .split_whitespace()
            .map(|s| {
                if self.do_lower_case {
                    BasicTokenizer::_run_strip_accents(s.to_lowercase())
                } else {
                    s.to_string()
                }
            })
            .flat_map(|c| BasicTokenizer::_run_split_on_punc(c))
            //.map(|p| p.to_string()).collect::<Vec<String>>()
            .filter(|s| s.len() > 0)
            .collect();

        tokens
    }
}

struct WordpieceTokenizer {
    vocab: IndexMap<String, usize>,
    inv_vocab: IndexMap<usize, String>,
    unk_token: String,
    unk_id: usize,
    max_input_chars_per_word: usize,
}

impl WordpieceTokenizer {
    pub fn new<T: AsRef<str>>(
        vocab: IndexMap<String, usize>,
        inv_vocab: IndexMap<usize, String>,
        unk_token: T,
        max_input_chars_per_word: usize,
    ) -> WordpieceTokenizer {
        let unk_id = vocab[unk_token.as_ref()];
        // RFC 1682
        WordpieceTokenizer {
            vocab,
            inv_vocab,
            unk_id,
            unk_token: unk_token.as_ref().to_string(),
            max_input_chars_per_word,
        }
    }

    pub fn tokenize<T: AsRef<str>>(&self, text: T) -> Vec<String> {
        let mut output_tokens = Vec::new();
        for token in text.as_ref().split_whitespace() {
            let chars: Vec<char> = token.chars().collect();
            if chars.len() > self.max_input_chars_per_word {
                output_tokens.push(self.unk_token.clone());
                continue;
            }
            let mut is_bad = false;
            let mut start = 0;
            let mut sub_tokens = Vec::new();
            while start < chars.len() {
                let mut end = chars.len();
                let mut cur_substr = String::new();
                while start < end {
                    let mut substr: String = chars[start..end].iter().collect();
                    if start > 0 {
                        substr = "##".to_string() + &substr;
                    }
                    if self.vocab.contains_key(&substr) {
                        cur_substr = substr;
                        break;
                    }
                    end -= 1;
                }
                if cur_substr.len() == 0 {
                    is_bad = true;
                    break;
                }
                sub_tokens.push(cur_substr);
                start = end;
            }
            if is_bad {
                output_tokens.push(self.unk_token.clone());
            } else {
                output_tokens.append(&mut sub_tokens);
            }
        }
        return output_tokens;
    }

    pub fn tokenize_to_ids<T: AsRef<str>>(&self, text: T) -> Vec<i32> {
        let mut output_tokens = Vec::new();

        for token in text.as_ref().split_whitespace() {
            let chars: Vec<char> = token.chars().collect();
            if chars.len() > self.max_input_chars_per_word {
                output_tokens.push(self.unk_id as i32);
                continue;
            }
            let mut is_bad = false;
            let mut start = 0;
            let mut sub_tokens = Vec::new();
            while start < chars.len() {
                let mut end = chars.len();
                let mut cur_substr = -1;
                while start < end {
                    let mut substr: String = chars[start..end].iter().collect();
                    if start > 0 {
                        substr = "##".to_string() + &substr;
                    }
                    if let Some(val) = self.vocab.get(&substr) {
                        cur_substr = *val as i32;
                        break;
                    }
                    end -= 1;
                }
                if cur_substr == -1 {
                    is_bad = true;
                    break;
                }
                sub_tokens.push(cur_substr);
                start = end;
            }
            if is_bad {
                output_tokens.push(self.unk_id as i32);
            } else {
                output_tokens.append(&mut sub_tokens);
            }
        }
        return output_tokens;
    }
}

pub struct FullTokenizer {
    basic_tokenizer: BasicTokenizer,
    wordpiece_tokenizer: WordpieceTokenizer,
    pub cls_token_id: usize,
    pub sep_token_id: usize,
    pub pad_token_id: usize,
    pub unk_token_id: usize,
    pub mask_token_id: usize
}

pub fn convert_tokens_to_ids(vocab: &IndexMap<String, usize>, tokens: &[String]) -> Vec<usize> {
    tokens
        .iter()
        .map(|k| vocab.get(k).unwrap().clone())
        .collect()
}

pub fn convert_ids_to_tokens(vocab: &IndexMap<usize, String>, ids: &[usize]) -> Vec<String> {
    ids.iter().map(|i| vocab.get(i).unwrap().clone()).collect()
}

impl FullTokenizer {
    fn load_vocab() -> Result<(IndexMap<String, usize>, IndexMap<usize, String>), Box<dyn Error>> {
        let raw_vocab_str = include_bytes!("../vocab.txt");
        let mut vocab = IndexMap::new();
        let mut inv_vocab = IndexMap::new();
        let mut index: usize = 0;

        for buffer in raw_vocab_str.split(|u| *u == b'\n') {
            {
                let token = String::from_utf8_lossy(&buffer).trim().to_string();
                vocab.insert(token.clone(), index);
                inv_vocab.insert(index, token);
                index += 1;
            }
        }

        Ok((vocab, inv_vocab))
    }

    pub fn new(do_lower_case: bool) -> Result<FullTokenizer, Box<dyn Error>> {
        let basic_tokenizer = BasicTokenizer::new(do_lower_case);

        let (vocab, inv_vocab) = FullTokenizer::load_vocab()?;

        if !vocab.contains_key("[CLS]")
            || !vocab.contains_key("[SEP]")
            || !vocab.contains_key("[UNK]")
        {
            return Err("`[CLS]` or `[SEP]` or `[UNK]` not in vocab".into());
        }

        let cls_token_id = *vocab.get("[CLS]").unwrap();
        let sep_token_id = *vocab.get("[SEP]").unwrap();
        let pad_token_id = *vocab.get("[PAD]").unwrap();
        let unk_token_id = *vocab.get("[UNK]").unwrap();
        let mask_token_id = *vocab.get("[MASK]").unwrap();
        let wordpiece_tokenizer = WordpieceTokenizer::new(vocab, inv_vocab, "[UNK]", 100);

        Ok(FullTokenizer {
            basic_tokenizer,
            wordpiece_tokenizer,
            cls_token_id,
            sep_token_id,
            pad_token_id,
            unk_token_id,
            mask_token_id
        })
    }

    pub fn tokenize<T: AsRef<str>>(&self, text: T) -> Vec<String> {
        let mut split_tokens = Vec::new();
        for token in self.basic_tokenizer.tokenize(text) {
            split_tokens.append(&mut self.wordpiece_tokenizer.tokenize(token));
        }
        split_tokens
    }

    pub fn tokenize_to_ids<T: AsRef<str>>(&self, text: T) -> Vec<i32> {
        let mut input_ids = vec![];
        for token in self.basic_tokenizer.tokenize(text) {
            input_ids.append(&mut self.wordpiece_tokenizer.tokenize_to_ids(token));
        }
        input_ids
    }

    pub fn convert_tokens_to_ids(&self, tokens: &[String]) -> Vec<usize> {
        convert_tokens_to_ids(&self.wordpiece_tokenizer.vocab, tokens)
    }

    pub fn convert_ids_to_tokens(&self, ids: &[usize]) -> Vec<String> {
        convert_ids_to_tokens(&self.wordpiece_tokenizer.inv_vocab, ids)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_punctuation() {
        assert!(_is_punctuation('-'));
        assert!(_is_punctuation('$'));
        assert!(_is_punctuation('`'));
        assert!(_is_punctuation('.'));

        assert!(!_is_punctuation('A'));
        assert!(!_is_punctuation(' '));
    }
    #[test]
    fn test_is_control() {
        assert!(_is_control('\u{0005}'));

        assert!(!_is_control('A'));
        assert!(!_is_control(' '));
        assert!(!_is_control('\t'));
        assert!(!_is_control('\r'));
    }

    #[test]
    fn test_is_whitespace() {
        assert!(' '.is_whitespace());
        assert!('\t'.is_whitespace());
        assert!('\r'.is_whitespace());
        assert!('\n'.is_whitespace());
        assert!('\u{00A0}'.is_whitespace());

        assert!(!'A'.is_whitespace());
        assert!(!'-'.is_whitespace());
    }

    #[test]
    fn test_convert_tokens_to_ids() {
        let vocab_tokens = [
            "[UNK]", "[CLS]", "[SEP]", "want", "##want", "##ed", "wa", "un", "runn", "##ing",
        ];

        let mut vocab = IndexMap::new();
        for (i, token) in vocab_tokens.iter().enumerate() {
            vocab.insert(token.to_string(), i);
        }
        let tokens: Vec<String> = ["un", "##want", "##ed", "runn", "##ing"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let ids = convert_tokens_to_ids(&vocab, &tokens);
        assert_eq!(ids, [7, 4, 5, 8, 9]);
    }

    #[test]
    fn test_wordpiece_tokenizer() {
        let vocab_tokens = [
            "[UNK]", "[CLS]", "[SEP]", "want", "##want", "##ed", "wa", "un", "runn", "##ing",
        ];
        let mut vocab = IndexMap::new();
        let mut inv_vocab = IndexMap::new();
        for (i, token) in vocab_tokens.iter().enumerate() {
            vocab.insert(token.to_string(), i);
            inv_vocab.insert(i, token.to_string());
        }
        let tokenizer = WordpieceTokenizer::new(vocab, inv_vocab, "[UNK]", 100);

        assert_eq!(tokenizer.tokenize("").len(), 0);

        {
            let expected: Vec<String> = ["un", "##want", "##ed", "runn", "##ing"]
                .iter()
                .map(|s| s.to_string())
                .collect();
            assert_eq!(tokenizer.tokenize("unwanted running"), expected);
        }

        {
            let expected: Vec<String> = ["[UNK]", "runn", "##ing"]
                .iter()
                .map(|s| s.to_string())
                .collect();
            assert_eq!(tokenizer.tokenize("unwantedX running"), expected);
        }
    }

    #[test]
    fn test_basic_tokenizer_no_lower() {
        let tokenizer = BasicTokenizer::new(false);

        let expected: Vec<String> = ["HeLLo", "!", "how", "Are", "yoU", "?"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(tokenizer.tokenize(" \tHeLLo!how  \n Are yoU?  "), expected);
    }

    #[test]
    fn test_basic_tokenizer_lower() {
        let tokenizer = BasicTokenizer::new(true);
        {
            let expected: Vec<String> = ["hello", "!", "how", "are", "you", "?"]
                .iter()
                .map(|s| s.to_string())
                .collect();
            assert_eq!(tokenizer.tokenize(" \tHeLLo!how  \n Are yoU?  "), expected);
        }
        {
            let expected: Vec<String> = vec!["hello".to_string()];
            assert_eq!(tokenizer.tokenize("H\u{00E9}llo"), expected);
        }
    }

    #[test]
    fn test_chinese() {
        let tokenizer = BasicTokenizer::new(true);
        let expected: Vec<String> = ["ah", "\u{535A}", "\u{63A8}", "zz"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(tokenizer.tokenize("ah\u{535A}\u{63A8}zz"), expected);
    }

    #[test]
    fn test_full_tokenizer() {}
}
