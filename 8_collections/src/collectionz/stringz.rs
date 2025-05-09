pub fn strings_example() {
    // Diffrence between '&str' and 'String'

    let s: &str = "hello";                          // String literals, stored in the program's
                                                    // Binary

    let s1 = String::from(" nama saya ican");       // Heap Allocated, mutable and ownable.

    println!("{s}{s1}");

    let data = "Shakh Rukh";                        // String literals
    let mut s = data.to_string();

    println!("{s}");
   
    let lastname = String::from(" Khan");
    s.push_str(&lastname);                           // Only take string slice from 'lastname'
    println!("{s}");

    // Concatenation with +

    let s1 = String::from("Kendrick");
    let s2 = String::from(" Lamar");
    let s3 = s1 + &s2;                              // s1 has been removed and ownership transfered
                                                    // into s3
    println!("{s3}");

    // We can also use format!
    
    let a = String::from("gunting");
    let b = String::from("batu");
    let c = String::from("kertas");

    let full = format!("{a} {b} {c}");              // We can use format, to concatenate strings

    println!("{full}");
    println!("{a}");                                // 'a' still exist
}
