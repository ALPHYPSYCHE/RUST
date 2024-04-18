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
