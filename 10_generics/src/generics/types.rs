use std::cmp::PartialOrd;
use num_traits:: Float;

struct koordinat<T> {
    x:T,
    y:T,
}

struct coordinates<T, U> {
    x: T,
    y: U,
}

struct coordinates_w_lat<P, A, C> {
    x: P,
    y: A,
    z: C,
}

struct coordinates_true<T: Float> {
    x: T,
    y: T,
}

// kita bisa menggunakan generic types di method seperti ini
impl<P, A, C> coordinates_w_lat<P, A, C> {
    fn z(&self) -> &C {
        &self.z
    }
}

// atau contoh berguna seperti ini
impl<T: Float> coordinates_true<T> {
    fn jarak(&self, other: coordinates_true<T>) -> T {
        let x = self.x - other.x;
        let y = self.y - other.y;

        (x.powi(2) + y.powi(2)).sqrt()
    }
}

pub fn type_example() {
    undplicating_i32_with_function();

    // But what if we want to create general function that takes in different data types?
    
    // GENERIC DATA TYPES
    // We can use the 'T' For it
   
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = largest_every_type(&number_list);     // See the fucntion below
   
    println!("{largest}");

    let rumah_ican = koordinat {x: 3, y: 5};
    // let rumah_ishanda = koordinat {x: 3, y: 5.0};    // gabisa karena structnya cuma <T>
    let rumah_ishanda = koordinat {x:3.0, y: 10.0};     // bisa karena data typenya sama
    let rumah_oscar = koordinat {x:"Jauh", y: "Banget"};// bahkan string aja bisa
    
    // klo mau beda data type pake <T, U>
    let rumah_jopari = coordinates {x: 10, y: 5.2324};
    let rumah_alvin = coordinates {x: 10, y: 10};       // bahkan bisa sama juga sich

    // Bahkan bisa lebih banyak jika mau nambahin <T, U, ....so on...>
    let rumah_cholil_erk = coordinates_w_lat {x: 20, y: 3.4234234, z:"Tinggi banget dah pokoknya"};

    println!("ketinggian rumah cholil erk: {}", rumah_cholil_erk.z());

    // implement method jarak (buat ego gua aja bisa buat)
    let rumah_ale = coordinates_true { x: 2.434, y: 3.5304 };
    let rumah_annya = coordinates_true { x: 3.4880780, y: 5.0739};

    println!("Jarak rumah annya dengan ale: {}", rumah_annya.jarak(rumah_ale));

    // OVERVIEW OF GENERIC TYPES
    // Using generic types doesn't slow your code because of 'monomorphization', it compiles both
    // on every type used in the program. Every generic type must be known at compile time.

}

fn largest_every_type<T: PartialOrd>(list: &[T]) -> &T {
    // The error message ask us to add "std::cmp::PartialOrd", it means only certain types you can
    // put into the function (in Appendix C of rust book). Which in our case it fits i32 and char

    // basiclly the same function as below  
    let mut largest = &list[0];

    for number in list{
        if number > largest {                   
            largest = number;
        }
    }

    largest
}

fn undplicating_i32_with_function() {
    let number_list = vec![34, 50, 25, 100, 65];
    let number_list2 = vec![34, 2, 233, 1000, 6];

    let largest_number = find_largest_number(&number_list); 
    let largest_number2 = find_largest_number(&number_list2);

    println!("The largest number is {largest_number} & {largest_number2}");
}

fn find_largest_number(list: &[i32]) -> &i32 {  // we want to return a reference to the list
    let mut largest = &list[0];

    for number in list{
        if number > largest {                   // largest already reference in the list no need to
                                                // add '&' again
            largest = number;
        }
    }

    largest
}

// We can create largest char
fn find_largest_char(list: &[char]) -> &char {  // we want to return a reference to the list
    let mut largest = &list[0];

    for number in list{
        if number > largest {                   // largest already reference in the list no need to
                                                // add '&' again
            largest = number;
        }
    }

    largest
}


