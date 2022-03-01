# pkg3

This is an example package.
It contains a single library crate, `lib.rs`, and multiple binary crates.

To see the binaries that this package provides, run:

```
cargo build --package pkg3 --bin
```

Build an individual binary using:

```
cargo build --package pkg3 --bin <NAME>
# for example,
cargo build --package pkg3 --bin pkg3
```

Build just the library crate using:

```
cargo build --package pkg3 --lib
```

Build everything in the package (all binary crates and the library crate) using:

```
cargo build --package pkg3
```

To test the package, run:

```
cargo test --package pkg3
```

Or to test individual crates:

```
cargo test --package pkg3 --bin <NAME>
cargo test --package pkg3 --lib
cargo test --package pkg3 --doc
```

Run individual binaries using:

```
cargo run --package pkg3 --bin pkg3
# or more simply
cargo run --bin pkg3
```
