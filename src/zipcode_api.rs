use anyhow::Result;
use serde::{Deserialize};

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct Address {
    zipcode: String,
    prefcode: String,
    address1: String,
    address2: String,
    address3: String,
}

#[derive(Deserialize, Debug)]
struct JsonSchema {
    status: i32,
    message: Option<String>, // may be null
    results: Vec<Address>,
}

pub struct Searcher{
}

const API_URL: &str = "https://zipcloud.ibsnet.co.jp/api/search";

impl Searcher {
    pub fn new() -> Searcher {
        Searcher{}
    }

    pub fn search(&self, zip_code:&str) -> Result<Vec<Address>>{
        let url = format!("{}?zipcode={}", API_URL, zip_code);
        let resp: JsonSchema = reqwest::blocking::get(url)?.json()?;
        Ok(resp.results)        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0(){
        let searcher = super::Searcher::new();
        let got = searcher.search("7830060").unwrap();
        let want = vec![super::Address::default()];
        assert_eq!(got.len(), want.len());
        if got.len() == want.len() {
            for i in 0..got.len(){
                assert_eq!(got[i], want[i]);
            }
        }
    }
}