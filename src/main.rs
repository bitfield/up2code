use std::{env, thread, time::Duration};

use up2code::{diff, listings};

fn main() -> anyhow::Result<()> {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("Usage: up2code [PATH, ...]");
        return Ok(());
    }
    for path in paths {
        for listing in listings(&path)? {
            let listing = listing.check()?;
            if let Some(diff) = diff(&listing.local, &listing.remote) {
                println!("{path}: {} - {}", listing.title, listing.url);
                println!("{diff}");
            };
            // Sleep to avoid hitting GitHub API rate limit
            thread::sleep(Duration::from_millis(1000));
        }
    }
    Ok(())
}
