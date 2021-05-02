fn add(number1: i32, number2: i32) -> i32 {
    return number1 + number2;
}

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}

fn main() {
    let x = 3;
    let y = 6;

    println!("{} + {} = {}", x, y, x + y);
    println!("{} * {} = {}", x, y, x * y);

    let num = add(12, 10);
    println!("{}", num);

    let str = "123";
    let num: i32 = str.parse().unwrap(); // 📌 Note: The use of .unwrap() is to “catch” the potential error and fail at this point.
    println!("{}", num);
}
