mod front_of_house {
    pub mod hosting {                       // Add pub keyword, because it's private in default 
        pub fn add_to_waitlist() {}         // Even though parent is public, child can be private
                                            // ensure to add pub also
    }
}

fn deliver_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();             // Using `super` to access parent level items
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();  // Absolute path, from 'crate'

    front_of_house::hosting::add_to_waitlist();         // Relative path, from current module 
}
