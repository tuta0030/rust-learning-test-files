use std::io;
use std::cmp::Ordering;
use rand::Rng;


// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value:i32) -> Guess {
//         if value <1 || value > 100 {
//             panic!("猜测的数字需要在1-100以内，但是得到了 {} ", value);
//         }
//         Guess { value}
//     }

//     pub fn value(&self) -> i32 {
//         self.value
//     }
// }


pub fn main() {
    println!("猜数游戏\n");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut index: i32 = 0;

    // print!("神秘数字是：{} \n", secret_number);

    loop {
        
        println!("输入一个数字来猜测是否正确：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess:i32 = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // 用自定义的结构体来实现数字范围的检查
        // let guess = Guess::new(guess);

        if guess < 1 || guess > 100 {
            println!("输入的数字应该在 1-100 之间");
            continue;
        }

        println!("你猜测的数是：{}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => print!(" < 神秘数字小于你的数字"),
            Ordering::Greater => println!(" > 神秘数字大于你的数字"),
            Ordering::Equal => {
                println!("神秘数字等于你的数字! 你赢了！");
                print!("\n你尝试的次数： {} \n", index+1);
                break;
            }
        }
        index += 1;
        print!("\n你尝试的次数：{}", index);
        print!("\n")
    }


}
