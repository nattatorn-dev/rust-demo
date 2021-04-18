mod conditionals;
mod loops;
mod numbers;
mod strings;
mod structs;
mod traits;
mod tuples;
mod vectors;

fn main() {
    // string
    strings::demo();

    // number
    numbers::demo();
    let num = numbers::add(12, 10);
    println!("{}", num);

    // tuple
    tuples::demo();

    // loop
    loops::demo();

    // vector
    let number1 = vec![1, 2, 3, 4, 5];
    let number2 = vec![5, 6, 7, 8, 9];
    let result1 = vectors::push(number1, number2);
    println!("{:?}", result1);

    let number3 = vec![1, 2, 3, 4, 5];
    let number4 = vec![5, 6, 7, 8, 9];
    let result2 = vectors::merge(number3, number4);
    println!("{:?}", result2);

    let user1 = structs::User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);

    let user2 = structs::build_user(String::from("admin@admin.com"), String::from("admin"));
    println!("{:?}", user2);

    let books = structs::Book::created();
    books.add(1);
    books.add(4);
    books.add(99);
    books.print();

    let schools = structs::School::created();
    let john = structs::Student {
        first_name: String::from("john"),
        last_name: String::from("legend"),
    };
    schools.add(john);
    schools.print();

    traits::demo();
    conditionals::demo();
}
