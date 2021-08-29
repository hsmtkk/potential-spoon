use anyhow::Result;

#[derive(Clone, Copy)]
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
}
