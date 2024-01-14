
fn main() {
    test_basic_if();
    test_loop_to_ten_return_twenty();
    test_while_loop();
    test_for_loop();
    test_for_range();
}

fn test_basic_if () {

    println!("调用基础的分支函数");
    let number = 3;

    if number < 5 {
        println!("小于5");
    } else {
        println!("不小于5");
    };
    println!("\n\n")
}


fn test_loop_to_ten_return_twenty () {
    print!("调用循环到10的函数");

    let mut counter = 0;
    let _result = loop {
        counter += 1;

        if counter == 10 {
            break counter*2;
        }
    };

    println!("The results is : {}", _result);
}

fn test_while_loop () {
    let mut a = 10;
    while a<20 {
        println!("当前的 a 是：{}", a);
        a += 1;
        
    }
}

fn test_for_loop () {
    let a = [1, 2, 3, 4, 5];
    for item in a.iter() {
        println!("for loop current value is: {}", item);
    }
}

fn test_for_range() {
    for item in (1..4).rev() {
        println!("{}",item);
    }
    println!("倒数结束")
}
