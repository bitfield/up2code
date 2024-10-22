**SOLUTION:** This is one way to do it:

\vspace{5pt}
```rust
#[test]
fn append_appends_line_to_existing_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("logbook.txt");
    fs::write(&path, "hello\n").unwrap();
    append(&path, "logbook").unwrap();
    let text = fs::read_to_string(path).unwrap();
    assert_eq!(text, "hello\nlogbook\n", "wrong text");
}
```
([Listing `logbook_4`](https://github.com/bitfield/tsr-tools/blob/main/logbook_4/src/lib.rs))
