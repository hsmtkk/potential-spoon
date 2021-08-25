use anyhow::Result;

pub struct Render {}

impl Render {
    pub fn new() -> Render{
        Render{}
    }

    pub fn render(&self) -> Result<String>{
        Ok("<html><body><p>Hello</p></body></html>".to_string())
    }
}
