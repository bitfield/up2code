use regex::Regex;
use similar::TextDiff;

use std::{env, fs, sync::LazyLock};

static LISTINGS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?m)^```rust\n(?<code>[^`]+?\n)?```\n\(\[Listing `(?<listing>[\s\S]+?)`\]\((?<link>.*?)\)",
    )
    .unwrap()
});

fn main() -> anyhow::Result<()> {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("Usage: up2code [PATH, ...]");
        return Ok(());
    }
    let http = reqwest::blocking::Client::builder().build()?;
    for path in &paths {
        let text = fs::read_to_string(path)?;
        for m in LISTINGS.captures_iter(&text) {
            let Some(code) = m.name("code") else { continue };
            let Some(listing) = m.name("listing") else {
                continue;
            };
            let Some(link) = m.name("link") else { continue };
            let raw_url = String::from(link.as_str());
            match http.get(raw_url + "?raw=true").send() {
                Err(e) => {
                    println!("{path}: {e}");
                }
                Ok(resp) => {
                    if let Err(e) = resp.error_for_status_ref() {
                        println!("{path}: {e}");
                        continue
                    } 
                    let text = resp.text()?;
                    if !text.contains(code.as_str()) {
                        println!("{path}: Listing {}", listing.as_str());
                        let diff = TextDiff::from_lines(code.as_str(), &text);
                        print!("{}", diff.unified_diff());
                    }
                }
            }
        }
    }
    Ok(())
}
