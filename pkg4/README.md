# pkg4

This package depends on other packages.
It illustrates how to call code from one package.

Useful commands:

```
cargo build --package pkg4
```

Run the binary provided by this package

```
cargo run --bin pkg4
```

Since this package only provides one binary,
it is possible to "run" the package directly
(Cargo does some magic)

```
cargo run --package pkg4
```
