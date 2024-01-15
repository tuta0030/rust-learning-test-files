fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); //使用相对路径，调用上级的函数
        crate::serve_order(); //使用绝对路径， 调用上级的函数
    }

    fn cook_order() {}
}

fn main () {}