![Ali Amirshahi Banner](https://i.ibb.co/2WKr9HR/github-banner-small.png)

<a href="https://github.com/ALPHYPSYCHE/RUST/blob/main/README.md">
    <div style="margin-bottom:1em;"> 
        <img style="margin-right:-.2em;" align="left" src="https://cdn.worldvectorlogo.com/logos/rust.svg" alt="rust)" title="RUST" width="100" height="100"/>
    </div>
    <div style="margin-bottom:-1.5em;">
        <h1 display="display:inline;">
            <font size="+4">Rust Tutorial</font>
        </h1>
    </div>
</a>

<div style="margin-left:5em;">
    <span style="vertical-align: middle;"><font size="+2">This tutorial will guide you through the basics of Rust programming. Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety—meaning that all references point to valid memory—without a garbage collector.</font></span>
</div>

 
## 📚 Table of Contents

1. [Hello World!](#hello-world!)
2. [Rust Tools](#rust-tools)
3. [Variables, Constants](#variables,-constants)
4. [Data Types](#data-types)
5. [References](#references)
6. [Import Libraries](#import-libraries)
7. [Console Input](#console-input)
8. [Arithmetic & Type Casting](#arithmetic-&-type-casting)
9. [Conditions and Control Flow](#conditions-and-control-flow)
10. [if / else if / else / match](#if-/-else-if-/-else)
11. [Functions, Expressions & Statements](#functions,-expressions-&-statements)
12. [Pointers](#pointers)
13. [Array](#array)
14. [Iteration](#iteration)
15. [Generics](#generics)
16. [Error Handling](#error-handling)
17. [Stack and Heap](#stack-and-heap)
18. [Ownership, Borrowing, and Lifetimes](#ownership,-borrowing,-and-lifetimes)
19. [Closures](#closures)
20. [HashMap](#hashMap)
21. [Structs and Enums](#structs-and-enums)
22. [Traits](#traits)
23. [Modules and Crates](#modules-and-crates)
24. [Smart Pointers / Box](#smart-pointers-/-box)
25. [Concurrency](#concurrency)




## 💠 Hello World!

  ```rust
fn main(){
    println!("HELLO WORLD! THIS IS MY 1ST LINE OF CODE IN RUST!")
}
```
run it by opening the terminal and run this: [.\main.exe] in Windows, or [./main.exe] in Mac/Linux


## 💠 Rust Tools (cargo, rustfmt)

::::: to start and make a new project with cargo :::::

open the terminal in your project folder and run this:
```bash
cargo new tutorial2
```

and then go to the folder and run this (will add some files)
```bash
cargo build 
```

and for debugging go to this folder:
```lua
02_project2\tutorial2\target\debug
```
and run this:
```bash
.\tutorial2
```
---------------------------
or you can run this command:
```bash
cargo run
```
you can check if you can compile the code and there is no error without running the code.
```bash
cargo check
```
for auto-correct the format in the code:
go to this folder:
..\02_project2\tutorial2\src

and run this:
```bash
rustfmt main.rs
```
---------------------------
.toml --> Toms, Obvious, Minimal, Language

you can add [dependencies] with this:
```bash
cargo add ...
```

## 💠 Variables, Constants

  ```rust
fn main() {
    //use "mut" for changing the parameter
    let mut x = 12; // its int
    println!("The value for X is : {}", x);

    x = x*3;
    println!("The value for X*3 is : {}", x);

    //or use like this with "let"
    let y = 10;
    println!("The value for Y is : {}", y);

    let y = y*10;
    println!("The value for Y*10 is : {}", y);

    {
    //for changing inside the {}: it will change the y only in the {} and outside the {} its the same value as outer of the {}
    let y = 4;
    println!("The value for Y (inside) is : {}", y);

    let y = y*10;
    println!("The value for Y*10 (inside) is : {}", y);
    }

    let y = y+1;
    println!("The value for Y+1 (from Y outside) is : {}", y);

    //change the type with "let"
    let y = "HELLO";
    println!("The value for y (String) is: {}", y);

    //const : you can not change constant all and must be constant
    const SEC_IN_MIN: u32 = 66; //UPPERCASE NAME for const with _ and no space
    const PI_NUMBER: f32 = 3.141592;
    println!("{}",SEC_IN_MIN);
}
```

## 💠 DATA TYPES

  ```rust
fn main() {
    println!("Tutorial 4 - data types");

    let x: i32 = 2; // signed integer number 32 bit - i8/i16/i32/i64/i128 / *default
        //i8 range --> -2^7 to 2^7 -1 --> + & - numbers [-128 to 127]

    let y: u32 = 5; // unassigned inteeger number 32 bit - u8/u16/u32/u64/u128
        //u8 range --> 0 to 2^8 -1 --> + numbers [0 to 255]

    let a: f32 = 3.14; // float number 32 bit / single precision / *default
    let a: f64 = 5.67; // float number 64 bit / double precision

    let true_or_false: bool = true; // Boolian true
    let true_or_false3: bool = false; // Boolian false

    let myword: char = "apple"; // charecter

    let tup: (i32,bool,char) = (1,true,"aa"); // tupple & they are immutable
    println!("{}", tup.0); // print 1st value of the tuple
    println!("{}", tup.1); // print 2nd value of the tuple
    let tup2: (i8, bool, char) = (1, true, "aa"); // you can not assign these 2 numbers because they have different type i32 & i8
    let mut tup3: (i32,bool,char) = (3,false,"bb"); // mutable tuple
    tup3.0 = 10; // only works with mutables & not work with immutables
    println!("{}", tup3.0); // print 1st value of the tuple
    tup3 = (14,true,"rr");
    println!("{}", tup3.0); 
    println!("{}", tup3.1);

    let arr = [1,2,3,4,5,6];
    arr[0]
    arr[1]

    let mut arr2: [i32; 6] = [1,2,3,4,5,6];
    arr[4]=3;
    println!("{}", arr[4]);

    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
    println!("Max usize: {}", usize::MAX);
}
```

## 💠 References(&)
References in Rust are lightweight and safe alternatives to pointers. They allow you to borrow values without taking ownership, enabling you to pass values around without transferring ownership.
References are commonly used when you want to pass data to functions without consuming or modifying it, or when you want to avoid the overhead of copying large data structures.
References ensure memory safety by enforcing the borrowing rules at compile time, preventing common pitfalls like dangling references or multiple mutable references to the same data.

## 💠 Import Libraries
  ```rust
use std::io; // standard input
use rand::Rng;
use std::io::{Write, BufRead, BufReader,ErrorKind};
use std::fs::File;

fn main() {
    println!("What is your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name).expect("Did Not Receive Input");

    println!("Hello {}! {}", name.trim_end(),greeting); 
```
## 💠 Console Input

Prelude :
the prelude is the list of things that Rust automatically imports into every Rust program.
It's kept as small as possible & is focused on things particularly traits that are used in almost every single rust program.

  ```rust
use std::io; // standard input

fn main() {
    println!("Tutorial 5 - console input");
    let mut input = String::new();

    // library     result object          expect
    io::stdin().read_line(&mut input).expect("fail to read line");

    // if .read_line(&mut input) give us valid value, and if it couldnt read .expect("fail to read line");
    println!("{}", input);


}
```
.read_line() --> the method to read lines. | 
.read_line(input) --> its a copy of the value & will not change the original value | 
.read_line(& input) -->  with & (ampersand) we use reference. reference in default is immutable | 
.read_line(&mut input) -->  The reference in default is immutable. so we change it so we can modify the value of the reference | 
.expect("fail to read line") --> will catch any error that occure. error handling | 

so when we run the code, it starts and waits for the user input and then it will print a copy of the input.

## 💠 Arithmetic & Type Casting

```rust
fn main() {
    println!("Tutorial 6 - Arithmetic & Type Casting");

    let x: u8 = 12; // u8 --> 0 - 255
    let y: i8 = 10; // i8 --> -128 - 127

    // or you can also write like these :
    let x= 12u8; // put u8 at the end
    let y= 10i8;

    let x= 12_u8; // put _u8 at the end
    let y= 10_i8;

    let x= 12 as u8; // put 'as u8' at the end
    let y= 10 as i8;

    let z = x + y; // it will not work because the types are different --> u8 , i8
    // the 'z' will be in 'i8' type
    println!("{}", z); // so we will get an error (Overflow)

    let a: u8 = 255; //0 - 255
    let b: u8 = 1; // -128 - 127

    let c = a + b; // it will not work because you can't set 256 in 255 space. so we should use bigger space like u16 or u32.
    println!("{}", c); // so we will get error (Overflow)

    let d: u8 = 255; //0 - 255
    let e: u8 = 10; // -128 - 127

    let f = d / e; // it will give you 25 . because it is an integer and does not give you 25.5 
    println!("{}", f);

    let g: f32 = 255.0; //0 - 255
    let h: f32 = 10.0; // -128 - 127
    
    // or you can also write like this :
    let g= 125.7f32;
    let h= 104.9f32;

    let i = g / h; // the 'i' will be in 'f32' type
    println!("{}", i);

    let j= 125_000i32; // 125_000 is the same as 125000
    let k= 10i32;

    let l = j / k; // the 'i' will be in 'i32' type
    println!("{}", l);

    let m= 125_000 as i64; // 125_000 is the same as 125000
    let n= 10i32; // the 'n' is  in 'i32' type

    let o = m / (n as i64); // the 'n' will be in 'i64' type //same as -->  let o = m / n as i64;
    println!("{}", o);

    let p= (i32::MAX as i64) + 1;
    let q= 10_i32; 

    let o = p as i32 /q; // you will not get the error, but you are overflowing and will give you negative numbers.
    println!("{}", o);
}
```


for parsing the input :

  ```rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("fail to read line");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 3)
}
```

## 💠 Conditions and Control Flow

```rust
fn main() {
    println!("Tutorial 7 - Conditions and Control Flow");

    // conditions :      < > <= >= != ==
    
    let cond = 2 <= 4; // boolian answer (True/False)

    // you can not use intiger and flout like this --> let cond = 2 <= 4.6; must be the same type
    // --> let cond = 2.0 <= 4.6   or let cond = (2 as f32) <= 4.6  
    
    println!("{}", cond);

    //logicals :         and: &&    or: ||    not: !

    let cond2 = true && cond; // see if left and right are true
    println!("{}", cond2);

    let cond3 = false || cond; // see if left or right are true
    println!("{}", cond3);

    let cond4 = !(false || cond); // ! will flip the output
    println!("{}", cond4);

    let cond5 = false && !cond;
    println!("{}", cond5);
}
```
1st use ! then && then ||

## 💠 if / else if / else / match

if statements are used for simple boolean conditions, while match expressions are more powerful and versatile, allowing pattern matching against different values or patterns. Use if for simple conditions and match for more complex pattern-matching scenarios.

```rust
fn main() {    
    let food = "cookie";
   
    if food == "cookie"{
        println!("I love cookies!")
    } else if food == "fruit" {
        println!("Thats healthy!")
    } else {
        println!("Sorry!!")
    }

    let food2 = "fruit";
   
    if food2 == "cookie"{
        println!("I love cookies!")
    } else if food2 == "fruit" {
        println!("Thats healthy!")
    } else {
        println!("Sorry!!")
    }
}
```
match example:
```rust
use std::io; // standard input
use rand::Rng;
use std::io::{Write, BufRead, BufReader,ErrorKind};
use std::fs::File;

fn main() {
    let my_age: i32 = 18;
    let voting_age: i32 = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't vote"), // use , instead of ; for match [at the end of the line]
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("1st vote"),
    };
}
```

## 💠 Functions, Expressions & Statements

Functions: Functions in Rust are blocks of code that perform a specific task. They are defined using the fn keyword followed by the function name, parameters, return type, and body. Functions can accept parameters, return values, and execute code when called. Rust function can return an expression, but not return a statement.

Expressions: Expressions in Rust are code that produces a value. They can be simple values, function calls, or combinations of operators and operands that evaluate to a value. Expressions can be used in various contexts, such as assigning values to variables, passing arguments to functions, or returning values from functions.
the expression will give you value. --> macro , function --> in 'let x = 20' the number 20 is an expression

Statements: Statements in Rust are instructions that perform actions but do not produce a value. Unlike expressions, statements are not evaluated to a value and are typically terminated with a semicolon ;. Examples of statements include variable declarations, function declarations, and control flow constructs like if, while, and for loops.
statement is like variable declaration --> let x = 20; --> it will not return any value.
so you can not do this --> let x = let y = 20;

In summary, functions define reusable blocks of code, expressions produce values, and statements perform actions. 


```rust
fn main() {
    println!("Tutorial 8 - Functions, Expressions & Statements");
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

//Another example of expression:
fn main2(){
    let number = {
        let x = 5;
        x + 2 // this have not ; at the end . so it will return value without; at the end.
    };
    println!(" the number change to : {}", number);
}

fn add_numbers_2(x: i32, y: i32) -> i32 { // '->' is the 'return value operator' in rust
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
```

## Simple Example

```rust
use std::io; // standard input
use rand::Rng;
use std::io::{Write, BufRead, BufReader,ErrorKind};
use std::fs::File;

fn main() {
    let arr_1: [i32; 4] = [1, 2, 3, 4];
    println!("1st : {}", arr_1[0]);
    println!("LLength : {}", arr_1.len());
}
```
## Simple Example

```rust
use std::io; // standard input
use rand::Rng;
use std::io::{Write, BufRead, BufReader,ErrorKind};
use std::fs::File;

﻿

fn main() {
    let arr_2: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx: usize = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx +=1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx +=1;
    }
}
```
## 💠 Pointers (*):
Pointers in Rust are used to directly interact with memory. They are primarily used when you need to manage memory manually, such as in low-level systems programming or when interfacing with C code.
You typically use pointers when you need to allocate memory dynamically or when you need to work with data at a low level where references are not appropriate.
Pointers allow for more flexibility and control over memory, but they also require careful management to avoid memory safety issues like null pointer dereferences or dangling pointers.

##  Good Example (let's review)

```rust
use rand::Rng;
use std::io;

fn main() {
    println!("Tutorial 9");
    println!(" ");
    println!("########### WELCOME TO MY GAME PLATFORM ###########");

    let name = greeting();
    let eligible = age(&name);

    if eligible {
        let mut balance = 100; 
        menu(&name, &mut balance);
    } else {
        println!(" ");
    }
}

fn greeting() -> String {
    println!(" ");
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you.";
    io::stdin().read_line(&mut name).expect("Did not receive input");
    println!(" ");
    println!("Hello {}! {}", name.trim(), greeting);
    name
}

fn age(name: &str) -> bool {
    println!(" ");
    println!("How old are you, {}?", name.trim());
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Did not receive input");
    let age: i32 = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid age.");
            return false; // if parsing fails
        }
    };
    if age >= 18 {
        println!(" ");
        println!("You are over 18. You can play.");
        true
    } else {
        println!(" ");
        println!("Sorry {}, you are not eligible to play.", name.trim());
        false
    }
}

fn menu(name: &str, balance: &mut i32) {
    loop {
        println!(" ");
        println!("{}, Please choose a game (1-4), or enter 'q' to quit:", name.trim());
        println!(" ");
        println!("1. Tic Tac Toe (18$)");
        println!("2. Hangman (15$)");
        println!("3. Sudoku (20$)");
        println!("4. Snake (16$)");
        println!(" ");
        println!("Your current account balance is: {} $", balance);

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                if *balance >= 18 {
                    println!(" ");
                    println!("** You chose 1: Tic Tac Toe! **");
                    *balance -= 18;
                    println!("You have been charged 18$. Your remaining balance is {} $", balance);
                    // Tic Tac Toe game
                } else {
                    println!(" ");
                    println!("Insufficient balance! Please choose another game or 'q' to quit.");
                }
            }
            "2" => {
                if *balance >= 15 {
                    println!(" ");
                    println!("** You chose 2: Hangman! **");
                    *balance -= 15;
                    println!("You have been charged 15$. Your remaining balance is {} $", balance);
                    // Hangman game
                } else {
                    println!(" ");
                    println!("Insufficient Balance! Please Recharge Your Balance First!");
                }
            }
            "3" => {
                if *balance >= 20 {
                    println!(" ");
                    println!("** You chose 3: Sudoku! **");
                    *balance -= 20;
                    println!("You have been charged 20$. Your remaining balance is {} $", balance);
                    // Sudoku game
                } else {
                    println!(" ");
                    println!("Insufficient balance! Please choose another game or 'q' to quit.");
                }
            }
            "4" => {
                if *balance >= 16 {
                    println!(" ");
                    println!("** You chose 4: Snake! **");
                    *balance -= 16;
                    println!("You have been charged 16$. Your remaining balance is {} $", balance);
                    // Snake game
                } else {
                    println!(" ");
                    println!("Insufficient balance! Please choose another game or 'q' to quit.");
                }
            }
            "q" => {
                println!(" ");
                println!("** Quitting... **");
                break;
            }
            _ => {
                println!(" ");
                println!("Invalid choice! Please enter a number between 1 and 4, or 'q' to quit.");
            }
        }
    }
}

```
## 💠 Array

In Rust, an array is a fixed-size collection of elements of the same data type stored in contiguous memory. Unlike vectors, which are also collections but dynamically resizable, arrays have a fixed length determined at compile time.

```rust

fn main() {
    println! ("Tutorial 10 - Array");
    println! ("-----------");
    let vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1,2,3,4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1){
        Some(second) => println!("2nd : {}", second), 
        None => println!("No 2nd value"),
    }
    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println! ("Vec Length {}", vec2.len()); 
    println!("Pop: {:?}", vec2.pop());
}

```
## 💠 Iteration
 Iteration refers to the process of sequentially accessing each element of a collection (such as an array, vector, or iterator) to perform some operation or computation on it. This typically involves using constructs like loops (e.g., for loops) or iterator methods (e.g., map, filter, fold) to iterate over the elements.
 
```rust
fn main() {
    println!(" ");
    println!("Tutorial 17 - Iteration ");
    println!("-----------------------------");

    let mut array_it = [1,2,3,4,5];
    for val in array_it.iter() {  // iterator is going to cycle through these values by borrowing those values.
        println!("{}", val);
    }
    let mut iter_2 = array_it.iter();
    println!("1st: {:?}",iter_2.next());

}
```

## 💠 Generics


Generics in Rust allow you to write code that can operate on different types without sacrificing type safety or performance. They enable you to write functions, structs, enums, and traits that work with any type, making your code more flexible and reusable.

in this example you can not just add (+) x and y . so we use generic add instead.
```rust
use std::ops::Add;

fn main() {
    println!("Int : 5 + 4 = {}", get_sum_generic(5, 4));
    println!("Float : 5.3 + 4.7 = {}", get_sum_generic(5.3, 4.7));
}

fn get_sum_generic <T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}


```

## 💠 Error Handling

In Rust, error handling is primarily done through the Result type for recoverable errors and the panic! macro for unrecoverable errors. Recoverable errors are handled using Result, which represents either a success (Ok) or a failure (Err). Unrecoverable errors cause the program to panic, terminating its execution. Rust encourages explicit error handling and propagation, promoting safer and more predictable code. The ? operator is used for error propagation within functions returning Results, allowing concise error handling.

Result:
The Result type is used to handle recoverable errors, meaning errors that your program can reasonably be expected to recover from. It is an enum defined as:

Ok(T): Represents a successful computation and contains the result value of type T.
Err(E): Represents a failed computation and contains an error value of type E.
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
By convention, the Ok variant is used for successful results, while the Err variant is used for errors. Example:

In this example, the divide function returns a Result where the success case contains the result of the division and the error case contains a string indicating division by zero.
```rust
fn divide(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("division by zero")
    } else {
        Ok(x / y)
    }
}

fn main() {
    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
```
panic!:
panic! is used to handle unrecoverable errors, such as logic errors or violations of assumptions. When a panic occurs, the program will unwind the stack, clean up resources, and exit. Example:
```rust
fn main() {
    let v = vec![1, 2, 3];
    println!("Item at index 10: {}", v[10]); // This will panic due to index out of bounds
}
```
Unrecoverable and Recoverable Errors:
Unrecoverable errors: These are errors that cannot be handled gracefully by the program, like accessing an index out of bounds. They typically cause the program to panic.
Recoverable errors: These are errors that your program can reasonably recover from, like failing to open a file. These errors are usually handled using Result.

Error Propagation:
Rust encourages explicit error handling and propagation. When a function encounters an error, it can return a Result, allowing the calling code to decide how to handle it. This promotes safer and more predictable code.

? Operator:
The ? operator is used to propagate errors. It can only be used within functions that return Result. If the expression evaluates to Err, the function will return immediately with that error. Otherwise, it extracts the value from Ok and continues execution. Example:

```rust
use std::fs::File;
use std::io::Read;

fn read_file_contents(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => println!("Error reading file: {}", err),
    }
}

```
In this example, if either File::open or file.read_to_string fails, the function will return an error immediately. Otherwise, it will return the file contents wrapped in Ok.

## Final Example ( + Read/Write Files )
```rust
use std::fs::File;
use std::io::{BufReader, BufRead, Write, ErrorKind};

fn main() {
    println!(" ");
    println!("Tutorial 16 - Error Handling ");
    println!("-----------------------------");
    
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file : {:?}", error);
        }
    };

    write!(output, "Hello my Friend!\nWhat is going on?").expect("Failed to write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("Random.text");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Random.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };
}
```

## 💠 Stack and Heap

Stack:
The stack is a region of memory used for storing variables and function call information in a last-in, first-out (LIFO) manner.
Variables with a known, fixed size and lifetime are typically allocated on the stack.
The stack is fast and efficient for memory allocation and deallocation since it involves simple pointer manipulation.
Stack memory is limited and has a fixed size determined by the operating system or the runtime environment.

Used for static memory allocation.
Variables have a fixed size and lifetime.
Managed automatically by the compiler.
Fast access and deallocation.
Limited in size.

Heap:
The heap is a region of memory used for dynamic memory allocation, where memory is allocated and deallocated at runtime.
Variables with an unknown size or lifetime, such as those created with Box<T> or Vec<T>, are typically allocated on the heap.
Memory allocation on the heap involves more complex bookkeeping and can be slower than stack allocation.
Heap memory is managed by the allocator, which keeps track of allocated and deallocated memory blocks to avoid memory leaks and fragmentation.

Used for dynamic memory allocation.
Variables have a dynamic size and lifetime.
Managed by the programmer.
Slower access and deallocation compared to the stack.
Size can grow dynamically during program execution.

In Rust, memory allocation on the heap is explicitly managed using smart pointers like Box<T> or data structures like Vec<T>, while stack allocation is managed implicitly by the compiler based on variable lifetimes and scopes. Rust's ownership and borrowing system helps ensure memory safety by enforcing strict rules for managing memory on both the stack and the heap.

## 💠 Ownership, Borrowing, and Lifetimes
ownership, borrowing, and lifetimes are fundamental concepts in Rust's memory management model. Ownership ensures memory safety by tracking resource ownership, borrowing enables temporary access to data without transferring ownership, and lifetimes help prevent dangling references by specifying the validity scope of references. These features collectively make Rust a safe and efficient language for systems programming.

Ownership: Ownership is a central concept in Rust that governs memory management. Every value in Rust has a single owner, and ownership rules ensure that memory is deallocated correctly and efficiently. Ownership rules prevent issues like dangling pointers, data races, and memory leaks by enforcing strict ownership transfer rules.

Borrowing: Borrowing allows you to temporarily loan a reference to a value without transferring ownership. In Rust, you can have either immutable borrows (&T) or mutable borrows (&mut T). Borrowing enables multiple parts of your code to access and operate on data without taking ownership, promoting code reuse and preventing unnecessary copying.

Lifetimes: Lifetimes are annotations that specify the scope for which references are valid. They help the Rust compiler ensure that borrowed references do not outlive the data they point to, preventing the use of invalid references. Lifetimes are expressed using apostrophe (') followed by a name and are often used in function signatures, structs, and trait definitions to specify the relationship between references and the data they borrow.

RULES:
1. Each value has a variable that's called its owner
2. There is only one owner at a time
3. When the owner goes out of scope the value disappears

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
another example (it will not run!)
you will get error becouse str1 is not exist anymore
```rust
fn main() {
    let str1: String = String::from("World");
    let str2: String = str1;
    println!("Hello {}", str1);
}
```
so you should use clone() to make a clone of str1
```rust
fn main() {
    let str1: String = String::from("World");
    let str2: String = str1.clone();
    println!("Hello {}", str1);
}
```

## Another example

```rust
fn print_str(x: String){
    println!("A string {}", x);  
}

fn print_return_str(x: String) -> String { 
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String){ 
    name.push_str(" is happy"); 
    println! ("Message : {}", name);
}

fn main() {
    println! ("Tutorial 11 - Ownership");
    println! ("-----------------------");
    let str1: String = String::from("World"); 
    let _str2: String = str1.clone();
    // if you active this line, print_str takes ownership of its argument, and str1 would be moved into print_str and no longer be valid for the rest of main
    //print_str(str1); 
    let str3: String = print_return_str(str1);
    println!("str3 = {}",str3);
}
```

## 💠 Closures
In Rust, closures are anonymous functions that can capture their enclosing environment, allowing them to access variables from the surrounding scope. They can be defined inline using the |params| { body } syntax and are often used for short, one-off operations or as arguments to higher-order functions like iterators. Closures provide a flexible way to create reusable blocks of code with access to their surrounding context.

```rust
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
```

## 💠 HashMap
A HashMap in Rust is a collection that maps keys to values, allowing for efficient lookup, insertion, and deletion based on the keys.

```rust
use std::collections::HashMap;

fn main() {
    println!("Tutorial 12 - HashMap");
    println!("-----------------------");

    let mut heroes: HashMap<&str, &str> = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    println!("Length : {}", heroes.len());

    if heroes.contains_key("Batman") {
        // Get value with key
        let the_batman: Option<&&str> = heroes.get("Batman");
        match the_batman {
            Some(_x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
}
```

## 💠 Structs and Enums

Structs:
short for structures, are custom data types that allow you to group multiple related pieces of data together.
They are defined using the struct keyword followed by a name and a list of fields.
Each field has a name and a type, similar to variables.
Structs enable you to create complex data structures with named fields, providing clarity and organization to your data.

Enums:
short for enumerations, allow you to define a type that can represent a set of named values.
Enums are useful for modeling different variants or states of data.
Each variant can optionally carry associated data of different types.
Enums are defined using the enum keyword followed by a name and a list of variants.

So, structs are used for grouping related data into custom types with named fields, while enums are used for defining types with a finite set of named values or variants. Both structs and enums are fundamental building blocks for creating custom data types and modeling complex data structures in Rust.

```rust
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
```

## Another Example + implement methods:

In Rust, impl is a keyword used to implement methods and associated functions for a particular trait or struct. It's used to define the behavior of types by providing implementations for traits or defining methods directly on structs. impl blocks allow you to encapsulate functionality and associate it with specific types, enabling code organization and reuse.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

## 💠 Traits

Traits in Rust are similar to interfaces in other languages and provide a way to define shared behavior across different types.

Definition: 
Traits define a set of methods that types can implement to share behavior.

Implementation: 
Types can implement traits to provide their own custom behavior for the methods defined in the trait.

Usage: 
Traits enable code reuse and polymorphism by allowing different types to share common functionality through trait implementations.

Syntax: 
Traits are defined using the trait keyword, and implementations are done using the impl keyword.

Associated Types and Methods: 
Traits can define associated types and methods, which allow for more flexible and generic behavior.

Default Implementations: 
Traits can provide default implementations for methods, which types can override if needed.

Trait Bounds: 
Functions and methods can specify trait bounds to constrain the types they accept, ensuring that only types implementing certain traits can be used.

In summary, traits in Rust enable code reuse, polymorphism, and generic programming by defining shared behavior that types can implement. They are a powerful feature of the language that promotes flexibility and composability in Rust code.

```rust
fn main() {
    println!("Tutorial 14 - Traits");
    println!("--------------------");

    const PI: f32 = 3.1415926535897;
    trait Shape {
        // This is a constructor which returns a Shape
        fn new(length: f32, width: f32) -> Self;

        // An area function that belongs to this trait
        fn area(&self) -> f32;
    }

    struct Rectangle {length: f32, width: f32};
    struct Circle {length: f32, width: f32};

    impl Shape for Rectangle{
        // Constructor
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }

        // self allows us to refer to parameters for this struct
        fn area(&self) -> f32{
            return self.length * self.width;
        }
    }

    // Implement the trait for circle
    impl Shape for Circle{
        // Constructor
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }

        fn area(&self) -> f32{
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    // Create circle and rectangle with Shape
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rec Area : {}", rec.area());
    println!("Circ Area : {}", circ.area());
}
```


## Another Example with Generic:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```
## 💠 Modules and Crates

Modules:
Modules in Rust are used to organize code into separate namespaces, making it easier to manage and maintain.
They allow you to group related items such as functions, structs, enums, and traits together.
Modules can be nested, enabling hierarchical organization of code.
They help in controlling visibility and access to items, allowing you to expose only the necessary parts of your code to other modules or crates.
Modules are declared using the mod keyword and are typically defined in separate files.

Crates:
Crates are the compilation units in Rust, containing one or more modules that form a library or executable.
A crate can be a library crate, which provides functionality for other crates to use, or an executable crate, which produces an executable binary.
Crates have a Cargo.toml file at their root, which specifies metadata and dependencies.
Crates can depend on other crates by specifying them in their Cargo.toml file, allowing for code reuse and modularity.
Crates can be published to the crates.io package registry for others to use.

In summary, modules organize code within a crate into separate namespaces, while crates are the compilation units that contain modules and can be libraries or executables. Modules and crates help in managing and organizing Rust code, promoting modularity, code reuse, and maintainability.

Modules: Organize & handle privacy
Crates: Modules that produce a library or executable
Packages: Build, test, & share crates
Paths: Naming an item such as a struct, function

```rust
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

fn main() {
    crate::sound::instrument::clarinet();
}
```

## Example (Tutorial 15 - Modules and Crates)

we have 2 files. 1st one is the main.rs & the 2nd one is mod.rs (in src/restaurant/mod.rs)

mod.rs (in src/restaurant/mod.rs)
```rust
pub mod pizza_order{
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: "Thin Crust Dough".to_string(),
                cheese: "Marinara Cheese".to_string(),
                topping: topping.to_string(),
            }
        }
    }

    pub mod helip_customer{
        pub fn seat_at_table(){
            println!("Hello, welcome to the seat at the table!");
        }
        pub fn take_order(){
            super::helip_customer::seat_at_table();
            let cust_pizza: super::Pizza = super::Pizza::lunch("Mozzarella");
            super::helip_customer::serve_customer(cust_pizza);
        }

        pub fn serve_customer(cust_pizza: super::Pizza){
            println!("Here is your pizza: ");
            println!("The customer is served a regular pizza with {}", cust_pizza.topping);
        }
    
    }
}

pub fn order_food(){
    pizza_order::helip_customer::take_order();
}
```
main.rs
```rust
mod restaurant;
use restaurant::order_food;

fn main() {
    println!(" ");
    println!("Tutorial 15 - Modules and Crates ");
    println!("---------------------------------");

    order_food();
}
```
## 💠 Smart Pointers / Box

In Rust, Box is a smart pointer that allocates memory on the heap and allows you to store data of a fixed size. It provides ownership and allows you to move values between different parts of your program. Box is commonly used when you need to store data with a known size dynamically or when you want to transfer ownership of a value to a different part of your program.

```rust

fn main() {
    println!(" ");
    println!("Tutorial 19 - Smart Pointers / Box ");
    println!("-----------------------------------");

    // BOX stores data on the HEAP, instead of the Stack (LIFO)
    // HEAP: request a certain amount of space, and the OS is going to find that amount of space somewhere in memory and allow you to store that information

    let box_int1 = Box::new(19);
    println!("box_int1 = {}", box_int1);

    // Create a Binary tree
    // We can't include nodes in a node because the size of a node depends on the size of multiple nodes which confuses the compiler

    // this one will gives you an error:

    // struct TreeNode<T> {
    //     pub left_hand: TreeNode<T>,
    //     pub right_hand: TreeNode<T>,
    //     pub key: T,
    //     }
    

    // Rust doesn't like Null values, so we have to use Option.
    
    struct TreeNode<T> {
        pub left_hand: Option<Box<TreeNode<T>>>,
        pub right_hand: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left_hand: None,
                right_hand: None,
                key: key,
            }
        }

        pub fn left_hand(mut self, node: TreeNode<T>) -> Self {
            self.left_hand = Some(Box::new(node));
            self
        }

        pub fn right_hand(mut self, node: TreeNode<T>) -> Self {
            self.right_hand = Some(Box::new(node));
            self
        }
    }

    let root = TreeNode::new(1)
    .left_hand(TreeNode::new(2))
    .right_hand(TreeNode::new(3));
}
```
## 💠 Concurrency

Concurrency refers to the ability to execute multiple tasks simultaneously. Concurrent programming involves designing and coordinating these tasks to run concurrently, typically to improve performance or responsiveness. Rust provides tools such as threads, channels, and async/await syntax to facilitate concurrent programming while ensuring memory safety and preventing data races through its ownership and borrowing system.

Common problems with parallel programming involve :
1. Threads are accessing data in the wrong order
2. Threads are blocked from executing because of confusion
3. over requirements to proceed with execution

```rust
use std::thread;
use std::time::Duration;

fn main() {
    println!(" ");
    println!("Tutorial 20 - Concurrency ");
    println!("-----------------------------------");

    // Create a thread with spawn
    
    thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);

            // Forces thread to sleep and allow another thread to execute
            thread::sleep(Duration::from_millis(1));
        }
    });

    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);

            // Forces thread to sleep and allow another thread to execute
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    thread1.join().unwrap();
}
```

## Another example

In this example:

1. We define a counter variable as a shared mutable data protected by a Mutex to ensure safe concurrent access.
2. We create 10 threads, each of which increments the counter.
3. Each thread acquires a lock on the counter using lock() before modifying it, ensuring that only one thread can access the counter at a time.
4. After all threads finish execution, we print the final value of the counter.
   
Common problems with parallel programming, such as data races and synchronization issues, are mitigated in Rust through features like ownership, borrowing, and synchronization primitives like Mutex. However, it's still important to be mindful of potential issues and design concurrent programs carefully.

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // Shared mutable data protected by a Mutex
    let counter = Arc::new(Mutex::new(0));

    // Create several threads to increment the counter concurrently
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Acquire the lock to access and modify the shared data
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Access the final value of the counter
    println!("Final counter value: {:?}", *counter.lock().unwrap());
}
```
