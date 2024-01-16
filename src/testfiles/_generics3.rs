enum Option<T> {
    Some(T),
    None,
}

enum Result < T, E > {
    Ok(T),
    Err(E),
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub fn main () {
    let p = Point {x:5, y:10};
    println!("p.x = {}", p.x());
}