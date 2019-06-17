mod front_of_house {
    pub mod hosting { // public module does not make contents public
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }

    pub mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::serving::serve_order();
        }

        fn cook_order() {}

        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // aboslute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}

pub fn eat_during_summer() {
    let mut meal = front_of_house::back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("Id like {} toast please", meal.toast);

    // will not compile, no access to seasonal_fruit
    //meal.seasonal_fruit = String::from("blueberries");
}