mod back_of_house {
    // 结构体是公共的，里面的字段 fields 是私有的
    pub struct  Breakfast {
        pub toast: String,
        seasonal_fruit: String,
        
    }

    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant () {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    meal.seasonal_fruit = String::from("Blueberries");
}

pub fn main() {
}