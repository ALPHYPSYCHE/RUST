![Ali Amirshahi Banner](https://i.ibb.co/2WKr9HR/github-banner-small.png)

<a href="https://github.com/ALPHYPSYCHE/RUST/blob/main/README.md">
    <div style="margin-bottom:1em;"> 
        <img style="margin-right:-.2em;" align="left" src="https://cdn.worldvectorlogo.com/logos/rust.svg" alt="php)" title="PHP" width="100" height="100"/>
    </div>
    <div style="margin-bottom:-1.5em;">
        <h1 display="display:inline;">
            <font size="+4">Rust Tutorial</font>
        </h1>
    </div>
</a>

<div style="margin-left:5em;">
    <span style="vertical-align: middle;"><font size="+2">This tutorial will guide you through the basics of Rust programming.</font></span>
</div>
﻿

>>> Tutorial 1:

fn main(){
    println!("HELLO WORLD! THIS IS MY 1ST LINE OF CODE IN RUST!")
}
// run it by open terminal and run this: [.\main.exe] in windows, or [./main.exe] in mac/linux

------------------------------------------------------------------------------
------------------------------------------------------------------------------

>>> Tutorial 2: Rust Tools (cargo, rustfmt)

::::: to start and make a new project with cargo :::::

open terminal in your project folder and run this:
--> cargo new tutorial2

and then go to the folder and run this (will add some files)
--> cargo build 

and for debugging go to this folder:
....\02_project2\tutorial2\target\debug

and run this:
--> .\tutorial2
---------------------------
or you can run this command:
--> cargo run

you can check if you can compile the code and there is no error without running the code.
--> cargo check

for auto correct the format in the code:
go to this folder:
....\02_project2\tutorial2\src

and run this:
--> rustfmt main.rs
---------------------------
.toml --> Toms, Obvious, Minimal, Language

------------------------------------------------------------------------------
------------------------------------------------------------------------------

>>> Tutorial 3: Variables, Constants

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
    println!("The value for y (String) is : {}", y);

    //const : you can not change constat all and must be constant
    const SEC_IN_MIN: u32 = 66; //UPPERCASE NAME for const with _ nad no space
    println!("{}",SEC_IN_MIN);
}

------------------------------------------------------------------------------
------------------------------------------------------------------------------

>>> Tutorial 4: DATA TYPES

fn main() {
    println!("Tutorial 4");

    let x: i32 = 2; // signed integer number 32 bit - i8/i16/i32/i64/i128 / *default
        //i8 range --> -2^7 to 2^7 -1 --> + & - numbers [-128 to 127]

    let y: u32 = 5; // unassigned inteeger number 32 bit - u8/u16/u32/u64/u128
        //u8 range --> 0 to 2^8 -1 --> + numbers [0 to 255]

    let a: f32 = 3.14; // float number 32 bit / single precision / *default
    let a: f64 = 5.67; // float number 64 bit / double precision

    let true_or_false: bool = true; // Boolian true
    let true_or_false2: bool = 1; // Sometimes like this : Boolian true
    let true_or_false3: bool = false; // Boolian false
    let true_or_false4: bool = 0; // Sometimes like this : Boolian false

    let myword: char = "apple"; // charecter

    let tup: (i32,bool,char) = (1,true,"aa"); // tupple & they are immutable
    println!("{}", tup.0); // print 1st value of the tupple
    println!("{}", tup.1); // print 2nd value of the tupple
    let tup2: (i8,bool,char) = (1,true,"aa"); // you can not assign these 2 numbers because they have different type i32 & i8
    let mut tup3: (i32,bool,char) = (3,false,"bb"); // mutable tupple
    tup3.0 = 10; // only works with mutables & not work with immutables
    println!("{}", tup3.0); // print 1st value of the tupple
    tup3 = (14,true,"rr");
    println!("{}", tup3.0); 
    println!("{}", tup3.1);

    let arr = [1,2,3,4,5,6];
    arr[0]
    arr[1]

    let mut arr2: [i32; 6] = [1,2,3,4,5,6];
    arr[4]=3;
    println!("{}", arr[4]);
}

------------------------------------------------------------------------------
------------------------------------------------------------------------------

>>> Tutorial 5: Console Input

Prelude :
the prelude is the list of things that rust automatically imports into every rust program.
its kept as small as possible & is focused on things particularly traits which are used in almost every single rust program.

use std::io; // standard input

fn main() {
    println!("Tutorial 5");
    let mut input = String::new();

    // librarty     result object          expect
    io::stdin().read_line(&mut input).expect("fail to read line");

    // if .read_line(&mut input) give us valid value, and if it couldnt read .expect("fail to read line");
    println!("{}", input);

    //.read_line() --> the mothod to read lines.
    //.read_line(input) --> its a copy of the value & will not change the original value
    //.read_line(& input) -->  with & (ampersand) we use reference.reference in defaullt is immutale.
    //.read_line(&mut input) -->  the reference in defaullt is immutale. so we change it so we can modify the value of the reference.
    //.expect("fail to read line") --> will catch any error that occure. error handleing
}

// so when we run the code, it start and wait foe the uer input and then it will print the copy of the input.

------------------------------------------------------------------------------
------------------------------------------------------------------------------

>>> Tutorial 6:

