struct Point<T, U> {
    x: T,
    y: U,
}

pub fn main () {
    let integer = Point {x: 5, y: 1.0};
    let float = Point {x: 1.0, y: 4.0};
}