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
4. [DATA TYPES](#data-types)
5. [References](#references)
6. [Import Libraries](#import-libraries)
7. [Console Input](#console-input)
8. [Arithmetic & Type Casting](#arithmetic-&-type-casting)
9. [Conditions and Control Flow](#conditions-and-control-flow)
10. [if / else if / else](#if-/-else-if-/-else)
11. [Functions, Expressions & Statements](#functions,-expressions-&-statements)
12. [Pointers](#pointers)
13. [Array](#array)
14. [Generics](#generics)
15. [Error Handling](#error-handling)
16. [Stack and Heap](#stack-and-heap)
17. [Ownership, Borrowing, and Lifetimes](#ownership,-borrowing,-and-lifetimes)
18. [HashMap](#hashMap)
19. [Structs and Enums](#structs-and-enums)
20. [Modules and Crates](#modules-and-crates)
21. [Traits and Generics](#traits-and-generics)




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

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            println!("File not found, creating a new file!");
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating the file: {:?}", e),
            }
        }
        Err(error) => {
            panic!("Error opening the file: {:?}", error)
        }
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

## 💠 Modules and Crates

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

## 💠 Traits and Generics

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


