use anyhow::Result;

#[derive(Clone)]
pub struct Searcher {
    host: String,
}

impl Searcher {
    pub fn new(host:&str) -> Searcher {
        Searcher {host:host.to_string()}
    }

    pub fn search(&self, zipcode: &str) -> Result<String> {
        let url = format!("https://{}/api/search?zipcode={}", self.host, zipcode);
        let json = reqwest::blocking::get(url)?.text()?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
}
