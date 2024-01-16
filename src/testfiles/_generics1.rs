// #TODO

fn largest<T: PartialOrd> (number_list: &[T]) -> T {
    let mut largest = &number_list[0];
    for number in number_list {
        if number > largest {
            largest = number
        }
    }
    *largest
}

pub fn main () {
    let num_list = vec![30, 25, 53, 124];
    let largest_num = largest(&num_list);
    println!("在这个数列中 {:?} 最大的数字是 {} ", num_list, largest_num);
    let num_list2 = vec![ "Banana", "Apple", "Orange", "Peach"];
    // 将 Vec<String> 转换为 Vec<&str>
    let num_list2_as_strs: Vec<&str> = num_list2.iter().map(|s| s.as_ref()).collect();
    let largest_num2 = largest(&num_list2_as_strs);
    // let largest_num2 = largest(&num_list2);
    println!("在这个数列中 {:?} 最大的数字是 {} ", num_list2, largest_num2);

}