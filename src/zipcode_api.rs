use anyhow::Result;

pub struct Searcher {}

const API_URL: &str = "https://zipcloud.ibsnet.co.jp/api/search";

impl Searcher {
    pub fn new() -> Searcher {
        Searcher {}
    }

    pub fn search(&self, zipcode: &str) -> Result<String> {
        let url = format!("{}?zipcode={}", API_URL, zipcode);
        let json = reqwest::blocking::get(url)?.text()?;
        Ok(json)
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
