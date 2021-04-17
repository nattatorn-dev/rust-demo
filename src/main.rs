mod number;
mod schools;
mod string;
mod users;
mod vector;

fn main() {
    // string
    string::string();

    // number
    number::number();
    let num = number::add(12, 10);
    println!("{}", num);

    // vector
    let number1 = vec![1, 2, 3, 4, 5];
    let number2 = vec![5, 6, 7, 8, 9];
    let result1 = vector::push(number1, number2);
    println!("{:?}", result1);

    let number3 = vec![1, 2, 3, 4, 5];
    let number4 = vec![5, 6, 7, 8, 9];
    let result2 = vector::merge(number3, number4);
    println!("{:?}", result2);

    let user1 = users::User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);

    let user2 = users::build_user(String::from("admin@admin.com"), String::from("admin"));
    println!("{:?}", user2);

    let books = users::Book::created();
    books.add(1);
    books.add(4);
    books.add(99);
    books.print();

    let schools = schools::School::created();
    let john = schools::Student {
        first_name: String::from("john"),
        last_name: String::from("legend"),
    };
    schools.add(john);
    schools.print();
}
