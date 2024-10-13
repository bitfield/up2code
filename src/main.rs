use regex::Regex;
use std::{env, fs, sync::LazyLock};

static LISTINGS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?m)^```rust\n(?<code>[^`]+?)\n?```\n\(\[Listing `(?<listing>[\s\S]+?)`\]\((?<link>.*?)\)",
    )
    .unwrap()
});

fn main() -> anyhow::Result<()> {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("Usage: up2code [PATH, ...]");
        return Ok(());
    }
    for path in &paths {
        let text = fs::read_to_string(path)?;
        // println!("{text}");
        LISTINGS.captures_iter(&text).for_each(|m| {
            if let Some(code) = m.name("code") {
                println!("Code: {}", code.as_str());
            }
            if let Some(listing) = m.name("listing") {
                println!("Listing: {}", listing.as_str());
            }
            if let Some(link) = m.name("link") {
                println!("Link: {}", link.as_str());
            }
        });
    }
    Ok(())
}
