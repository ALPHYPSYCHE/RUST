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
