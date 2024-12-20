use anyhow::{Error, Result};
use reqwest;

pub struct AOCClient {
    client: reqwest::Client,
}

impl AOCClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    fn get_html_for_day(&self, day: usize) -> Result<String> {
        let result =
            reqwest::blocking::get(&format!("https://adventofcode.com/2024/day/{}?/input", day))?;

        if result.status().as_u16() == 404 {
            return Err(Error::msg("404"));
        }

        Ok(result.text()?)
    }

    pub fn get_description(&self, day: usize) -> Result<String> {
        let html = self.get_html_for_day(day)?;

        let captures = regex::Regex::new(r"(?s)<main>(?<main>.*)<\/main>")?
            .captures(&html)
            .ok_or_else(|| Error::msg("Failed to capture regex"))?;

        Ok(captures
            .name("main")
            .ok_or_else(|| Error::msg("Main not found"))?
            .as_str()
            .to_string())
    }

    pub fn get_day_title(&self, day: usize) -> Result<String> {
        let html = self.get_html_for_day(day)?;

        let captures = regex::Regex::new(r"<h2>-+ Day \d+: (?<title>.*)\ -+<\/h2>")?
            .captures(&html)
            .ok_or_else(|| Error::msg("Failed to capture regex"))?;

        Ok(captures
            .name("title")
            .ok_or_else(|| Error::msg("Title not found"))?
            .as_str()
            .to_string())
    }
}
