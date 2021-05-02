fn main() {
    println!("Hello, world!");

    let mut str = "Hola".to_string(); // 📌 Note: The use of mut when defining the type makes the variable mutable, this is required if you want to change the variable after initialization, otherwise it is a constant.
    str = str + " mundo";
    println!("{}", str);
    println!("{}", str.len());

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);
}
