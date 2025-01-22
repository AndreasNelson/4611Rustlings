// TODO: Fix the compiler error.
fn main() {
    let mut x = 3; // made x mutable so it can be updated in line 6
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
