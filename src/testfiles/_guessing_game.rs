use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn main() {
    println!("猜数游戏\n");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut index: i32 = 0;

    // print!("神秘数字是：{} \n", secret_number);

    loop {
        
        println!("输入一个数字来猜测是否正确：");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        println!("你猜测的数是：{}", guess);

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
