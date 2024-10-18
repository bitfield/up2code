[![Crate](https://img.shields.io/crates/v/up2code.svg)](https://crates.io/crates/up2code)
[![Docs](https://docs.rs/up2code/badge.svg)](https://docs.rs/up2code)
![CI](https://github.com/bitfield/up2code/actions/workflows/ci.yml/badge.svg)
![Audit](https://github.com/bitfield/up2code/actions/workflows/audit.yml/badge.svg)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# up2code

`up2code` is a tool for checking code listings in Markdown files, to make sure they're up to date with, and in sync with, canonical versions stored in a GitHub repo.

### Installation

```sh
cargo install up2code
```

### Usage

Run:

```sh
up2code book/*.md
```

`up2code` reads all the Markdown files you specify, looking for what it considers a “listing”: a fenced code block immediately followed by a web link. For example:

    ```rust
    fn main() {
        println!("Hello, world!")
    }
    ```
    [Listing `hello/1`](https://github.com/bitfield/example/blob/src/main.rs))

It will try to fetch the raw code page from the specified URL (appending "?raw=true"), reporting any errors. If the fetch succeeds, it will check that the Markdown listing is an exact substring of the GitHub listing, reporting any mismatch as a unified diff:

```
tests/data/test.md: Listing `counter_2`
@@ -6,8 +13,8 @@

     #[test]
     fn count_lines_fn_counts_lines_in_input() {
-        let input = io::Cursor::new("line 1\nline2\n");
+        let input = io::Cursor::new("line 1\nline 2\n");
         let lines = count_lines(input);
-        assert_eq!(2, lines);
+        assert_eq!(lines, 2);
     }
 }
```
