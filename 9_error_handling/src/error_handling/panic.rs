pub fn panic_example() {
    // panic!("it was good run init");

    let v = vec![1, 2, 3];

    println!("{}", v[99]);                  // run with "RUST_BACKTRACE=1 cargo run"
}
