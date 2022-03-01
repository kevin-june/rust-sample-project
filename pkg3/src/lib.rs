/// Print a greeting.
pub fn print_hello() {
    println!("Hello! This is in pkg3.");
}

/// The meaning of life, the universe, and everything according to `pkg3`.
/// Doesn't agree with `pkg2`, but everyone has their own perception of the universe.
///
/// Documentation can contain tests:
///
/// ```
/// assert_eq!(43, pkg3::the_meaning())
/// ```
pub fn the_meaning() -> usize{
    43
}

// Library crates can have their own tests.
#[cfg(test)]
mod library_tests {
    use super::*;

    #[test]
    fn what_is_the_meaning() {
        assert_eq!(43, the_meaning());
    }
}
