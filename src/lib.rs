//! `up2code` is a tool for checking code listings in Markdown files, to make
//! sure they're up to date with, and in sync with, canonical versions stored in
//! a GitHub repo.
//!
//! `up2code` reads all the Markdown files you specify, looking for what it
//! considers a “listing”: a fenced code block immediately followed by a web
//! link. For example:
//!
//! ```text
//!     ```rust
//!     fn main() {
//!         println!("Hello, world!")
//!     }
//!     ```
//!     [Listing `hello/1`](https://github.com/bitfield/example/blob/src/main.rs))
//! ```
//!
//! It will try to fetch the raw code page from the specified URL (appending
//! "?raw=true"), reporting any errors. If the fetch succeeds, it will check
//! that the Markdown listing is an exact substring of the GitHub listing,
//! reporting any mismatch as a unified diff:
//!
//! ```text
//! tests/data/test.md: Listing `counter_2`
//! @@ -6,8 +13,8 @@
//!
//!      #[test]
//!      fn count_lines_fn_counts_lines_in_input() {
//! -        let input = io::Cursor::new("line 1\nline2\n");
//! +        let input = io::Cursor::new("line 1\nline 2\n");
//!          let lines = count_lines(input);
//! -        assert_eq!(2, lines);
//! +        assert_eq!(lines, 2);
//!      }
//!  }
//! ```

use regex::Regex;
use similar::TextDiff;

use std::{fs, io, path::Path, sync::LazyLock};

static LISTINGS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^```.*?\n(?<code>[^`]+?\n)?```\n\(\[(?<title>[\s\S]+?)\]\((?<link>.*?)\)")
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

/// A listing as parsed out of the Markdown source.
pub struct Listing {
    pub title: String,
    pub code: String,
    pub url: String,
}

impl Listing {
    /// Fetches the canonical version of the listing from its GitHub URL.
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
            text: resp.text()?,
        })
    }
}

/// A checked listing containing both the Markdown version and the canonical
/// version from GitHub.
pub struct CheckedListing {
    pub title: String,
    pub code: String,
    pub text: String,
}

impl CheckedListing {
    #[must_use]
    /// Diffs the Markdown listing against its canonical GitHub version.
    pub fn diff(&self) -> Option<String> {
        if self.text.contains(&self.code) {
            None
        } else {
            let diff = TextDiff::from_lines(&self.code, &self.text);
            Some(format!("{}", diff.unified_diff()))
        }
    }
}
