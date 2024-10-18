use regex::Regex;
use similar::TextDiff;

use std::{fs, io, path::Path, sync::LazyLock};

static LISTINGS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?m)^```.*?\n(?<code>[^`]+?\n)?```\n\(\[(?<title>[\s\S]+?)\]\((?<link>.*?)\)",
    )
    .unwrap()
});

static HTTP: LazyLock<reqwest::blocking::Client> =
    LazyLock::new(|| reqwest::blocking::Client::builder().build().unwrap());

/// Returns all the listings in the file at `path`.
///
/// # Errors
///
/// Any errors returned by [`fs::read_to_string`].
pub fn listings(path: impl AsRef<Path>) -> io::Result<Vec<Listing>> {
    let mut listings = Vec::new();
    let text = fs::read_to_string(path)?;
    for m in LISTINGS.captures_iter(&text) {
        let Some(code) = m.name("code") else { continue };
        let Some(title) = m.name("title") else {
            continue;
        };
        let Some(link) = m.name("link") else { continue };
        let url = String::from(link.as_str());
        listings.push(Listing {
            title: String::from(title.as_str()),
            code: String::from(code.as_str()),
            url: String::from(url.as_str()),
        });
    }
    Ok(listings)
}

pub struct Listing {
    pub title: String,
    pub code: String,
    pub url: String,
}

impl Listing {
    /// Fetches the canonical listing from its URL and stores the text.
    ///
    /// # Errors
    ///
    /// Any errors returned by the `reqwest` client.
    pub fn check(self) -> reqwest::Result<CheckedListing> {
        let resp = HTTP.get(self.url.clone() + "?raw=true").send()?;
        resp.error_for_status_ref()?;
        Ok(CheckedListing {
            title: self.title,
            code: self.code,
            text: resp.text()?
        })
    }
}

pub struct CheckedListing {
    pub title: String,
    pub code: String,
    pub text: String,
}

impl CheckedListing {
    #[must_use]
    pub fn diff(&self) -> Option<String> {
        if self.text.contains(&self.code) {
            None
        } else {
            let diff = TextDiff::from_lines(&self.code, &self.text);
            Some(format!("{}", diff.unified_diff()))
        }
    }
}
