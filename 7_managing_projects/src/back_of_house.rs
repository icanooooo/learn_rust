fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}

fn cook_order() {}

pub struct Breakfast {              
    pub toast: String,
    seasonal_fruit: String,         // this will not follow struct privacy
}                                   

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("Peaches"), // Private items can only be accessed with
                                                     // items in the same level
        }
    }
}

pub enum Desserts {
    IceCream,               // All of this are automatically public, unlike structs
    Cake,
    Mousse,
}
