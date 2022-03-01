# pkg1

This is an example package.
It contains only a binary crate, `main.rs`.

This binary crate automatically inherits the name of the package:

```
cargo build --bin pkg1
```

Additionally, this package can be built independently:

```
cargo build --package pkg1
```

Run the binary using:

```
cargo run --bin pkg1
```
