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
