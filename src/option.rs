fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        // Result is wrapped in a `Some` variant
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        }
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    let age: Option<u8> = None;
    
    match age {
        Some(age) => {
            if age >= 21 {
                println!("can have beer");
            } else {
                println!("can not have beer, only {}", age);
            }
        },
        None => println!("unknown age")
    }
}
