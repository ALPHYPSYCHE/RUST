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
