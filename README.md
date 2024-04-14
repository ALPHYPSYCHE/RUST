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
5. [Import Libraries](#import-libraries)
6. [Console Input](#console-input)
7. [Arithmetic & Type Casting](#arithmetic-&-type-casting)
8. [Conditions and Control Flow](#conditions-and-control-flow)
9. [if / else if / else](#if-/-else-if-/-else)
10. [Functions, Expressions & Statements](#functions,-expressions-&-statements)
11. [Error Handling](#error-handling)
12. [Ownership, Borrowing, and Lifetimes](#ownership,-borrowing,-and-lifetimes)
13. [Structs and Enums](#structs-and-enums)
14. [Modules and Crates](#modules-and-crates)
15. [Traits and Generics](#traits-and-generics)




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
    println!("Tutorial 4");

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
    println!("Tutorial 5");
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
    println!("Tutorial 6");

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
    println!("Tutorial 7");

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

rust function can return an expression, but not return a statement.
statement is like variable declaration --> let x = 20; --> it will not return any value.
so you can not do this --> let x = let y = 20;
the expression will give you value. --> macro , function --> in 'let x = 20' the number 20 is an expression

```rust
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

//Another example of expression:
fn main2(){
    let number = {
        let x = 5;
        x + 2 // this have not ; at the end . so it will return value without; at the end.
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

##  Example

```rust
use rand::Rng;
use std::io;

fn main() {
    greeting();
    age();
}

fn greeting() {
    println!(" ");
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you.";
    io::stdin()
        .read_line(&mut name)
        .expect("Did not receive input");
    println!(" ");
    println!("Hello {}! {}", name.trim(), greeting);
}

fn age() {
    println!(" ");
    println!("How old are you?");
    let mut age = String::new();
    let okk = "You are over 18.";
    let notokk = "Sorry! You are under 18.";
    io::stdin()
        .read_line(&mut age)
        .expect("Did not receive input");
    let age: i32 = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid age.");
            return;
        }
    };
    if age >= 18 {
        println!(" ");
        println!("Hmm... ! {}", okk);
        random_id_gen();
        println!(" ");
    } else {
        println!(" ");
        println!("Hmm... ! {}", notokk);
        println!(" ");
    }
}

fn random_id_gen() {
    let random_id_num = rand::thread_rng().gen_range(10000..99999);
    println!("Your new ID number is: {}", random_id_num);
    println!(" ");
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

## 💠 Ownership, Borrowing, and Lifetimes

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


