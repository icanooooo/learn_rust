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

    // In rust, String is a wrapper over a u8 Vector
    let hello = String::from("Hello");
    println!("{}", hello.len());                    // This will result in 5 length.
                                                    // Each char have 1 bytes
    let hello_in_russia = String::from("Здравствуйте");
    println!("{}", hello_in_russia.len());          // This have 24 Length!?
                                                    // This is because each Unicode scalar value in
                                                    // the string take 2 bytes of storages.
                                                    // (Because non-ASCII).

    // Therefore we can't index it like hello_in_russia[0] or hello[3], it will create a compile
    // time error.

    let hindi_word = "नमस्ते";                        // Even this 4 character in hindi:
    println!("{}", hindi_word.len());               // Is 18 bytes

    // println!("{}", &hindi_word[0..2]);           Do this with caution, slicing strings with
                                                    // ranges could crash our program
    // Methods to reiterating a string
    for c in hindi_word.chars() {                   // reiterating in per-character
        println!("{}", c);
    }

    for c in hindi_word.bytes() {                   // reiterating it per-bytes
        println!("{}", c);
    }
}










