fn largest (number_list: &[i32]) -> i32 {
    let mut largest = number_list[0];
    for &number in number_list {
        if number > largest {
            largest = number
        }
    }
    largest
}

pub fn main () {
    let num_list = vec![30, 25, 53, 124];
    let largest_num = largest(&num_list);
    println!("在这个数列中 {:?} 最大的数字是 {} ", num_list, largest_num);
    let num_list2 = vec![ 2321, 23123, 1123, 213];
    let largest_num2 = largest(&num_list2);
    println!("在这个数列中 {:?} 最大的数字是 {} ", num_list2, largest_num2);

}