fn main() {
    println!(" ");
    println!("Tutorial 18 - Closures ");
    println!("-----------------------");

    // closures is a function without a name. 
    // parameters must be between |  |
    // let var_name = |param1, param2| -> return_type {Body of function}

    let can_vote = |age: u8| {
        age >= 18
    };

    println!("can 20 years human vote? : {}",can_vote(20));
    println!("can 14 years human vote? : {}",can_vote(14));

    //closures can access variables outside of its body (unlike functions).

    // Access variables outside of its body with borrowing
    let mut samp1 = 7;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();
    samp1 = 15;

    // Change values, if you mark the closure mutable
    let mut add_1 = || samp1 += 1;
    add_1();
    println!("samp1 = {}", samp1);
    samp1 = 9;
    println!("samp1 = {}", samp1);

    // Pass closures to functions (with generic)
    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 { 
        func(a, b)
    }

    let sum = |a, b| a + b;
    let multiple = |a, b| a * b;

    println!("3 + 4 = {}", use_func(3, 4, sum));
    println!("3 * 4 = {}", use_func(3, 4, multiple));
    
}
