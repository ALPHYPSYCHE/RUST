fn main() {
    println!("Tutorial 13 - Structure");
    println!("-----------------------");
    struct Customer{
        name: String,
        address: String,
        balance: f32,
    }

    let mut ali = Customer {
        name: String::from("Ali Amirshahi"),
        address: String::from("369 Main St"),
        balance: 120340.50
    };

    ali.address = String::from("369 Main St");
    println!("Name : {}", ali.name);
    println!("Address : {}", ali.address);
    println!("Balance : {} $", ali.balance);
}
