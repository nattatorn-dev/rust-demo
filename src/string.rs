fn greeting(name: String) -> String {
    return "Hello, ".to_string() + &name;
}

#[test]
fn test_greeting() {
    assert_eq!(greeting("John".to_string()), "Hello, John")
}

fn main() {
    println!("Hello, world!");

    let mut str = "Hola".to_string(); // 📌 Note: The use of mut when defining the type makes the variable mutable, this is required if you want to change the variable after initialization, otherwise it is a constant.
    str = str + " mundo";
    println!("{}", str);
    println!("{}", str.len());

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    println!("{}", greeting("John".to_string()));
}
