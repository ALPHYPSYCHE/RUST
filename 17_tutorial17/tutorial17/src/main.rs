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
