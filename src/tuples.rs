pub fn demo() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);
}
