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

Just as a refresher, here's the existing test for `count_lines` (we won't worry about the error-handling test for now):

\vspace{5pt}
```rust
#[test]
fn count_lines_fn_counts_lines_in_input() {
    let input = Cursor::new("line 1\nline 2\n");
    let lines = count_lines(input);
    assert_eq!(lines, 2, "wrong line count");
}
```
([Listing `count_2`](https://github.com/bitfield/tsr-tools/blob/main/count_2/src/lib.rs))

So, over to you again.
