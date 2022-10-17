struct Square<T: SquareTypes> {
    sideLength: T
}
struct Triangle<T: TriangleTypes> {
    side1Length: T,
    side2Length: T
}
struct Pyramid</*T*//*: PyramidTypes*/> {

}
// Restricting square possible types
pub trait SquareTypes {}
impl SquareTypes for u32 {}
impl SquareTypes for f64 {}
impl SquareTypes for String {}

// Restricting triangle possible types
pub trait TriangleTypes {}
impl TriangleTypes for f64 {}

// Restricting pyramide possible types
/*
pub trait PyramidTypes {}
impl PyramidTypes for Square<T>{}
impl PyramidTypes for Triangle<T>{}
*/

// ----------------------
// Square implementations
// ----------------------
impl Square<u32> {
    fn new(sideLength: u32) -> Square<u32> {
        Square {sideLength: sideLength}
    }

    fn area(&self) -> u32 {
        self.sideLength.pow(2)
    }
}

impl Square<f64> {
    fn new(sideLength: f64) -> Square<f64> {
        Square {sideLength: sideLength}
    }

    fn area(&self) -> f64 {
        self.sideLength.powf(2.)
    }
}

impl Square<String> {
    fn new(sideLength: &str) -> Square<f64> {
        Square {sideLength: String::from(sideLength).parse::<f64>().unwrap()}
    }
}
// ----------------------

// ------------------------
// Triangle implementations
// ------------------------
impl Triangle<f64> {
    fn new(side1Length: f64, side2Length: f64) -> Triangle<f64> {
        Triangle {side1Length: side1Length, side2Length: side2Length}
    }

    fn area(&self) -> f64 {
        (self.side1Length*self.side2Length)/2.
    }
}

fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6");

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1);
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}
