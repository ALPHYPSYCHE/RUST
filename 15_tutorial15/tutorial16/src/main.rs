mod restaurant;
use restaurant::order_food;

fn main() {
    println!(" ");
    println!("Tutorial 15 - Modules and Crates ");
    println!("---------------------------------");

    order_food();
}

    // Crates : Modules that produce a library or executable
    // Modules : Organize and handle privacy
    // Packages : Build, test and share crates
    // Paths : A way of naming an item such as a struct, function