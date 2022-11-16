# Billboard

Display informational boxes in the terminal.

## Example

Your `Cargo.toml` should include `billboard` as a dependency

```toml
[dependencies]
billboard = "0.2"
```

```rust
use billboard::Billboard;

fn main() {
  Billboard::default().eprint("Hello, World!");
}
```

More examples can be found [here](/examples).

## Acknowledgements

This library was heavily inspired by [boxen](https://npmjs.com/package/boxen),
but has no official association with that project.
