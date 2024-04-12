fn main() {
    println!("Tutorial 8");
    my_function();
    add_numbers(14, 11);
    main2();
    let result = add_numbers_2(16, 9);
    println!(" the result of add_numbers_2 is : {}", result);
    let mynum = mynumber();
    println!(" my number is : {}", mynum);
    let result2 = add_numbers_3(14, 21);
    println!(" the result of add_numbers_3 with return is : {}", result2);
    let result5 = add_numbers_4(13, 17);
    println!(" the result of add_numbers_4 is : {}", result5);
    let result7 = add_numbers_5(9, 8);
    println!(" the result of add_numbers_5 (will -12) is : {}", result7);
    let result8 = add_numbers_5(3, 4);
    println!(" the result of add_numbers_5  is : {}", result8);
}

// function --> name of the function must be snake_case & not CamelCase
fn my_function() {
    println!("This is my function!");
}

fn add_numbers(x: i32, y: i32) {
    println!("The sum is : {}", x + y)
}

// rust function can return an expression, but not return a statement.
//statement is like variable declaration --> let x = 20; --> it will not return any value.
// so you can not do this --> let x = let y = 20;
//expression will gives you value. --> macro , function --> in 'let x = 20' the number 20 is an expression

// another example of expression:
fn main2(){
    let number = {
        let x = 5;
        x + 2 // this have not ; at the end . so it will return value without ; at the end.
    };
    println!(" the number change to : {}", number);
}

fn add_numbers_2(x: i32, y: i32) -> i32 { // '->' is the 'return operator' in rust
    x + y // without ; at the end.
}

fn mynumber() -> i32 {
     33
}

fn add_numbers_3(x: i32, y: i32) -> i32 {
    return x + y // you can use ; at the end or not --> return x + y;
}

fn add_numbers_4(x: i32, y: i32) -> i32 {
    let result4 = x + y;
    result4
}

fn add_numbers_5(x: i64, y: i64) -> i64 {
    let result6 = x + y;
    if result6 > 12 {
        return result6 - 12;
    }else{
        return result6;
    }
}





