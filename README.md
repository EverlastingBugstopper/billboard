# Boxx

_note: this library is still in early stages and updates may contain breaking changes_

A library that displays informational boxes in the terminal.

## Example

Your `Cargo.toml` should include `boxx` as a dependency

```toml
boxx = "0.0.0-alpha"
```

```rust
use boxx::Boxx;

fn main() {
  Boxx::default().display("Hello, World!");
}
```

More examples can be found [here](/examples).

## Acknowledgements

This library was heavily inspired by [boxen](https://npmjs.com/package/boxen), but has no official association with that project.
