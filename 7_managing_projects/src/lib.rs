mod front_of_house {
    pub mod hosting {                       // Add pub keyword, because it's private in default 
        pub fn add_to_waitlist() {}         // Even though parent is public, child can be private
        pub fn confirm_booking() {}    
    }

    pub mod waitering {
        pub fn giving_menu() {}
    }

    pub mod valet {
        pub fn get_keys() {} 
    } 
}
fn deliver_order() {}

mod back_of_house {
    // USING `super` TO ACCES PARENT LEVEL FOR RELATIVE PATHS
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();             // Using `super` to access parent level items
    }
    fn cook_order() {}
   
    // STRUCTS AND ENUM PUB
    // STRUCTS
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,             // if not pub, seasonal_fruit is private
                                            // it will not follow the struct privacy 
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),    // Only can be accessed with items in
                                                            // the same level
            }
        }
    }

    // ENUMS 
    
    pub enum Desserts {
        IceCream,               // Every variants in an enum will follow its privacy
        Cake,
        Mousse,
    }
    
}

// 'use' KEYWORD TO SHORTEN THE CALL

use crate::front_of_house::hosting;
use crate::front_of_house::waitering::giving_menu as GivingMenu; // You can use `as` the create a
                                                                 // new name
pub use crate::front_of_house::valet;           // by adding pub you help enable others bring it to
                                                // the scope and others to use this module (if this
                                                // is written in a module).

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();  // Absolute path, from 'crate'

    front_of_house::hosting::add_to_waitlist();         // Relative path, from current module 

    // 'use' KEYWORD EXAMPLE
        
    hosting::confirm_booking();                         // You don't need to call the whole path
                                                        // if it's already called once using the
                                                        // 'use' keyword.
    // please note that 'use' are only can be used in the same-scope and/or the higher level.
    // Except if it's module you have to called it again even if in the same script.
    // it's a convention to called the parent rather than only the item

    GivingMenu();

    // STRUCT PUB EXAMPLE
    let mut meal = back_of_house::Breakfast::summer("Whole Grain");     // Directly using method on
                                                                        // creation
    meal.toast = String::from("sour bread");
    println!("I'd like some {} please", meal.toast);    // meal.seasonal_fruit cannot be accessed
                                                        // here because it's private
                                                        // We can only access it using the method,
                                                        // like inte `meal` variable example.


    // ENUM PUB EXAMPLE

    let icanos_dessert = back_of_house::Desserts::IceCream;
}
