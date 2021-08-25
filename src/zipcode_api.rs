use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Address {
    zipcode: String,
    prefcode: String,
    address1: String,
    address2: String,
    address3: String,
}

impl Address {
    pub fn new(
        zipcode: &str,
        prefcode: &str,
        address1: &str,
        address2: &str,
        address3: &str,
    ) -> Address {
        Address {
            zipcode: zipcode.to_string(),
            prefcode: prefcode.to_string(),
            address1: address1.to_string(),
            address2: address2.to_string(),
            address3: address3.to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct JsonSchema {
    status: i32,
    message: Option<String>, // may be null
    results: Vec<Address>,
}

pub struct Searcher {}

const API_URL: &str = "https://zipcloud.ibsnet.co.jp/api/search";

impl Searcher {
    pub fn new() -> Searcher {
        Searcher {}
    }

    pub fn search(&self, zip_code: &str) -> Result<Vec<Address>> {
        let url = format!("{}?zipcode={}", API_URL, zip_code);
        let resp: JsonSchema = reqwest::blocking::get(url)?.json()?;
        Ok(resp.results)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0() {
        let searcher = super::Searcher::new();
        let got = searcher.search("7830060").unwrap();
        let want = vec![super::Address::new(
            "7830060",
            "39",
            "高知県",
            "南国市",
            "蛍が丘",
        )];
        assert_eq!(got.len(), want.len());
        if got.len() == want.len() {
            for i in 0..got.len() {
                assert_eq!(got[i], want[i]);
            }
        }
    }
}
