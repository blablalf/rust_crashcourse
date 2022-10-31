struct Square<T: SquareTypes> {
    side_length: T
}
struct Triangle<T: TriangleTypes> {
    side1_length: T,
    side2_length: T
}
struct Pyramid<T: PyramidTypes> {
    base: T,
    height: f64
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
pub trait PyramidTypes {}
impl PyramidTypes for Square<u32> {}
impl PyramidTypes for Square<f64> {}
impl PyramidTypes for Square<String> {}
impl PyramidTypes for Triangle<f64> {}

// ----------------------
// Square implementations
// ----------------------
impl Square<u32> {
    fn new(side_length: u32) -> Square<u32> {
        Square {side_length: side_length}
    }

    fn area(&self) -> u32 {
        self.side_length.pow(2)
    }
}

impl Square<f64> {
    fn new(side_length: f64) -> Square<f64> {
        Square {side_length: side_length}
    }

    fn area(&self) -> f64 {
        self.side_length.powf(2.)
    }
}

impl Square<String> {
    fn new(side_length: &str) -> Square<f64> {
        Square {side_length: String::from(side_length).parse::<f64>().unwrap()}
    }
}
// ----------------------

// ------------------------
// Triangle implementations
// ------------------------
impl Triangle<f64> {
    fn new(side1_length: f64, side2_length: f64) -> Triangle<f64> {
        Triangle {side1_length: side1_length, side2_length: side2_length}
    }

    fn area(&self) -> f64 {
        (self.side1_length*self.side2_length)/2.
    }
}
// ----------------------

// ------------------------
// Pyramid implementations
// ------------------------
impl Pyramid<Square<u32>> {
    fn new(square: Square<u32>, height: f64) -> Pyramid<Square<u32>> {
        Pyramid {base: square, height: height}
    }

    fn volume(&self) -> f64 {
        0.
    }
}

impl Pyramid<Square<f64>> {
    fn new(square: Square<f64>, height: f64) -> Pyramid<Square<f64>> {
        Pyramid {base: square, height: height}
    }

    fn volume(&self) -> f64 {
        0.
    }
}

impl Pyramid<Square<String>> {
    fn new(square: Square<String>, height: f64) -> Pyramid<Square<String>> {
        Pyramid {base: square, height: height}
    }

    fn volume(&self) -> f64 {
        0.
    }
}

impl Pyramid<Triangle<f64>> {
    fn new(triangle: Triangle<f64>, height: f64) -> Pyramid<Triangle<f64>> {
        Pyramid {base: triangle, height: height}
    }

    fn volume(&self) -> f64 {
        0.
    }
}
// ----------------------

// ------------------------
// Main
// ------------------------
fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6");

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1);
    let pyramid_square = Pyramid::<Square<u32>/*, f64*/>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>/*, f64*/>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}
