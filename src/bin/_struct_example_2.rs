pub fn main(){
    // let w = 30;
    // let l = 50;
    let rect1 = Rectangle{
        width: 30,
        length: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };

    let rect3 = Rectangle {
        width: 35,
        length: 55,
    };

    let square = Rectangle::square(20);

    println!("{}",rect1.can_hold(&rect2));
    println!("{}",rect1.can_hold(&rect3));

    // println!("{:#?}", rect);
    // println!("{:#?}", rect);
}


#[derive(Debug)]
struct Rectangle {
    width: i32,
    length: i32,
}

impl Rectangle {
    fn area(&self) -> i32{
        self.width * self.length
    }

    fn can_hold (&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square (size:i32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}
