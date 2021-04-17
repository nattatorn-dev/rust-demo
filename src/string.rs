pub fn string() {
    println!("Hello, world!");

    let mut str = "Hola".to_string();
    str = str + " mundo";
    println!("{}", str);
    println!("{}", str.len());
}
