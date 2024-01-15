pub fn main () {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("{}", i);
        // 这里在修改 i 里面的值的时候需要解引用 取借来的东西里面的值，不直接修改借来的 v[i]
        println!("{} +1 = {}", i, *i+1);
    }
}