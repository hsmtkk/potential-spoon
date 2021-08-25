use anyhow::Result;
use crate::zipcode_api::Address;
use log::{debug};
use tera::{Context, Tera};

pub struct Render {
    tera: Tera,
}

impl Render {
    pub fn new() -> Result<Render>{
        let tera = Tera::new("template/*.html")?;
        Ok(Render{tera})
    }

    pub fn render(&self, addresses:Vec<Address>) -> Result<String>{
        let mut context = Context::new();
        context.insert("addresses", &addresses);
        let html = self.tera.render("zipcode.html", &context)?;
        debug!("{}", html);
        Ok(html)
    }
}

#[cfg(test)]
mod tests {
    use crate::zipcode_api::Address;
    use std::io::Read;
    #[test]
    fn test0(){
        let mut f = std::fs::File::open("test/zipcode_want.html").unwrap();
        let mut want = String::new();
        f.read_to_string(&mut want).unwrap();

        let render = super::Render::new().unwrap();
        let addresses = vec![Address::new(
            "7830060",
            "39",
            "高知県",
            "南国市",
            "蛍が丘",
        )];        
        let got = render.render(addresses).unwrap();
        assert_eq!(want, got);
    }
}