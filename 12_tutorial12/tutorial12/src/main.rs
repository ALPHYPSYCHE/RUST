use std::collections::HashMap;

// A HashMap in Rust is a collection that maps keys to values, allowing for efficient lookup, insertion, and deletion based on the keys.

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
