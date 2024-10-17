The conventional answer to this in Rust is to put our unit tests inside a module (`mod tests`, let's say, though the name is up to us), and then protect that module with a `cfg(test)` attribute:

\vspace{5pt}
```rust
#[cfg(test)]
mod tests {
    // tests go here
}
```

`cfg` turns compilation on or off for the item it's attached to, based on the value of some expression. `cfg(test)` means the `tests` module will be compiled only if we're running in `cargo test` mode. Otherwise it will be ignored entirely, saving time, energy, and the planet.

### Anatomy of a test module

So here's what our test looks like once we've moved it into its own module:

\vspace{5pt}
```rust
#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn count_lines_fn_counts_lines_in_input() {
        let input = io::Cursor::new("line 1\nline2\n");
        let lines = count_lines(input);
        assert_eq!(2, lines);
    }
}
```
([Listing `counter_2`](https://github.com/bitfield/tsr-tools/blob/main/counter_2/src/lib.rs))

A module can have its own `use` declarations, which is handy since we often want to `use` things in tests that we don't need in the library itself (`std::io` in this example).
