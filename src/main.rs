mod number;
mod string;

fn main() {
    string::string();

    number::number();
    let num = number::add(12, 10);

    println!("{}", num);
}
