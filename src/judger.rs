use super::FullTokenizer;
use regex::RegexSet;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
struct Case {
    query: String,
    bot: String,
    val: i64,
}

#[derive(Deserialize)]
struct Inner {
    template: Vec<String>,
    prompt: String,
    rule: Vec<Case>,
}

struct RuleInner {
    query_set: RegexSet,
    bot_set: RegexSet,
    values: Vec<i64>,
}

impl RuleInner {
    pub fn from_inner(inner: &Inner) -> Self {
        let mut queries = Vec::with_capacity(inner.rule.len());
        let mut bots = Vec::with_capacity(inner.rule.len());
        let mut values = Vec::with_capacity(inner.rule.len());
        for case in &inner.rule {
            queries.push(case.query.clone());
            bots.push(case.bot.clone());
            values.push(case.val);
        }
        RuleInner {
            query_set: RegexSet::new(queries).unwrap(),
            bot_set: RegexSet::new(bots).unwrap(),
            values,
        }
    }
    pub fn get(&self, query: &str, bot: &str) -> i64 {
        let queries_matches: HashSet<_> = self.query_set.matches(query).into_iter().collect();
        if queries_matches.is_empty() {
            return -1;
        }
        let bots_matches: HashSet<_> = self.bot_set.matches(bot).into_iter().collect();
        if bots_matches.is_empty() {
            return -1;
        }
        let matches: Vec<_> = queries_matches.intersection(&bots_matches).collect();
        return self.values[*matches[0] as usize];
    }
}

struct Rule {
    template: Vec<String>,
    prompt: String,
    rule: RuleInner,
}

pub struct Judger {
    pub tokenizer: FullTokenizer,
    inner: Rule,
}

impl Judger {
    pub fn get(&self, query: &str, bot: &str) -> i64 {
        self.inner.rule.get(query, bot)
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
        let rule_inner = RuleInner::from_inner(&inner);
        let rule = Rule {
            template: inner.template,
            prompt: inner.prompt,
            rule: rule_inner,
        };
        Ok(Judger {
            tokenizer: FullTokenizer::new(false)?,
            inner: rule,
        })
    }
}

#[cfg(test)]
mod test {
    extern crate regex;
    #[test]
    fn test() {
        use self::regex::RegexSet;
        let regexes = [
            r"\w+".to_string(),
            "foo".to_string(),
            "foo".to_owned(),
            "^$".to_owned(),
        ];
        let re = RegexSet::new(&regexes).unwrap();
        {
            let matches: Vec<_> = re.matches("foobar").into_iter().collect();
            println!("{:?}", matches);
        }
        {
            let matches: Vec<_> = re.matches("").into_iter().collect();
            println!("{:?}", matches);
        }
    }
}
