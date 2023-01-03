use super::FullTokenizer;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
struct Inner {
    template: Vec<String>,
    prompt: String,
    rule: HashMap<String, i64>,
}
pub struct Judger {
    pub tokenizer: FullTokenizer,
    inner: Inner,
}

impl Judger {
    pub fn get(&self, query: &str) -> i64 {
        *self.inner.rule.get(query).unwrap_or(&-1)
    }
    pub fn input_size(&self) -> usize {
        return self.inner.template.len();
    }
    pub fn format(&self, q: &str, s: usize) -> String {
        self.inner.template[s].replace("{}", q)
    }

    pub fn prompt(&self) -> String {
        self.inner.prompt.clone()
    }
    pub fn new<T: AsRef<str>>(filename: T) -> Result<Judger, Box<dyn Error>> {
        let file = File::open(filename.as_ref())?;
        let reader = BufReader::with_capacity(4096, file);
        let inner: Inner = serde_json::from_reader(reader)?;
        Ok(Judger {
            tokenizer: FullTokenizer::new(true)?,
            inner,
        })
    }
}

