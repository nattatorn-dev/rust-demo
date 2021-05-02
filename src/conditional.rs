fn main() {
    let x = 13;
    let evenodd = if x % 2 == 0 { "even" } else { "odd" };
    println!("{}", evenodd);

    let s = "1a23";

    let num = match s.parse::<i32>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error parsing: {}", e);
            println!("Defaulting to 0");
            0
        }
    };
    println!("{}", num);

    let mut v = vec![1, 2, 3];
    let last = match v.pop() {
        Some(n) => n,
        None => {
            println!("None left");
            0
        }
    };
    println!("{}", last);
}
