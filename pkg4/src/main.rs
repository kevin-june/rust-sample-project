// Expose a symbol from pkg2 for direct use.
use pkg2::the_meaning;

/// This crate depends on several other packages.
/// It illustrates how to call code from another package (library).
fn main() {
    println!("Hello, world!");

    // Cargo.toml specifies what packages are depended on
    // This automagically makes the packages available.
    pkg2::print_hello();
    pkg3::print_hello();

    // The above "use" statement imports one function from pkg2.
    // The function can be called directly without specifying the package.
    let meaning = the_meaning();
    println!("The meaning: {}", meaning);

    // Warning - don't get confused by a similar function in pkg3!
    let meaning = pkg3::the_meaning();
    println!("Also the meaning: {}", meaning);
}
