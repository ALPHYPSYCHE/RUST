
fn main() {
    println! ("Tutorial 10 - Array");
    println! ("-----------");
    let vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1,2,3,4]; // Declare an array
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
