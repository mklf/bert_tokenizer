
use std::error::Error;

use tcsivr::Judger;
fn main() -> Result<(), Box<dyn Error>>{
    let judger = Judger::new("/home/svn/frankfangli/code/rust/bert_tokenizer/ffi/x.json")?;


    Ok(())
}