use std::collections::HashMap;

pub fn hashmaps_example() {
    // HASH MAPS
    // Usefull for storing dynamic key-values data. Structure of data may not be known until
    // runtime. Unlike Structs, Hash Maps are dynamic, the key-value pair are not predetermined.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);                // Key-value pair type are determined in
                                                            // first insert
    scores.insert(String::from("Red"), 14);
    scores.insert(String::from("TYLER THE CREATOR"), 30);
    
    // ACCESSING HASH MAPS VALUE with key
    let team_name = String::from("TYLER THE CREATOR");      
    let score = scores.get(&team_name).copied().unwrap_or(0);   // Will be 0 if key not found

    println!("{score}");

    // OWNERSHIP IN HASH MAPS
    let nama = String::from("ican");
    let poin = 35;

    scores.insert(nama, poin);
    // variabel 'nama' dan 'poin' tidak berlaku karena ownership dipindahkan ke scores

    // OVERWRITING HASH MAPS

    scores.insert(String::from("TYLER THE CREATOR"), 1500); // This will overwrite previous value

    // Using 'entry' and 'or_insert' for inserting value if key is not present
    
    scores.entry(String::from("Lalalalava, ciciciciken")).or_insert(2); // Will insert because new
                                                                        // key
    scores.entry(String::from("Blue")).or_insert(13);                   // Will not insert because
                                                                        // there's already blue

    for (a, b) in &scores {
        println!("{a}: {b}");
    }

    // UPDATING VALUE BASED ON NEW VALUE 
  
    // Getting old score plus added score
    let mut new_score = scores.get(&team_name).copied().unwrap_or(0);
    new_score += 750;

    // Inserting to hashmap
    scores.insert(team_name, new_score);
   
    // Show updated score
    let current_score = scores.get("TYLER THE CREATOR").copied().unwrap_or(0);
    println!("Tyler: {current_score}");

    // We can create a function if we want to update the hashmap, but that's for another day.
}
