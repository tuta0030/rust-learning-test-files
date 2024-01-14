
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: &str::from("jokerisfun@126.com"),
        username: &str::from("tuta0030"),
        active: true,
        sign_in_count: 001,
    };

    println!("用户{}的用户名为：{}",user1.sign_in_count, user1.username);
}

// 好像不管用，但是不知道为啥
// fn test_struct_use_parameter_as_fields (username:String, email: String) -> User {
//     struct User {
//         username,
//         email,
//         number: i32,
//     }
// }
