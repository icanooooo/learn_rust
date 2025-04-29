fn main() {
    let mut s = String::from("Apple");  // Disimpan di heap

    println!("{}", &s);
    let a = &mut s;                     // Minjem refrence mutable di 's'

    a.push_str(", jeruk");              // kita ubah 's' tesebut melalui a

    println!("{}", &s);                 // Karena a minjem reference dari s, jika a diubah maka s
                                        // juga.
                                        // karen disini 's' disimpan secara immutable lagi, maka a
                                        // kini sudah tiada
    let b = &s;                         

    println!("{}", b);                  // kalau coba masukin 'a' akan error, karena sudah dipinjam
                                        // secara immutable lagi sebelumnya
    
    // Jadi peraturannya ada empat
    test_function(&mut s);               // Operasi apapun didalam ini tidak akan mengubah 's' yang
                                        // asli

}

fn test_function(s: &mut String) {
    s.push_str(", dan durian");  

    println!("{s}");
}


