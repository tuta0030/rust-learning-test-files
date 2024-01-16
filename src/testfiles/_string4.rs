pub fn main () {
    let mut s = String::from("foo");
    let s1 = String::from("bar");
    s.push_str(&s1);

    println!("{}", s);
}