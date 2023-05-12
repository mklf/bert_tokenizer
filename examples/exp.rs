use std::error::Error;

struct Base {
    a: i32,
}
impl Base {
    pub fn echo(&self) {
        println!("{}", self.a);
    }
}

struct Der(Base);

impl Der {
    pub fn new() -> Self {
        Der(Base { a: 1 })
    }
    pub fn echo(&self) {
        self.0.echo();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let der = Der::new();
    der.echo();
    Ok(())
}
