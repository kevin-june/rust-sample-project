/// Cargo references this crate (and all others within `bin/`) by filename.
fn main() {
    println!("This is another binary from pkg3");
}

// Binary crates can have their own tests
#[cfg(test)]
mod binary_tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
