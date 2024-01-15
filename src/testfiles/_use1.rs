
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist () {}
        fn some_function () {}
    }
}

use crate::front_of_house::hosting; // 在作用域里面引用 hosting，让作用域内都可以直接使用 hosting

pub fn eat_at_restaurant () {
    // 惯用做法是引入上级模块，不直接引用函数，这样可以明确哪些函数是调用的
    // 函数以外的 东西可以直接调用，例如 struct
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // hosting::some_function(); // 没法使用私有的函数
}

pub fn main(){}