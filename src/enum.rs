use std::fmt;

#[derive(Debug)]
enum Status {
    Online,
    Offline,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Orange,
    Custom(String)
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match &*self {
        Color::Red => write!(f, "#FF0000"),
        Color::Green => write!(f, "#008000"),
        Color::Blue => write!(f, "#0000FF"),
        Color::Orange => write!(f, "#FFA500"),
        Color::Custom(color) => write!(f, "{}", color)
      }
  }
}

fn main() {
    let status = Status::Online;
    println!("{:?}", status);
    println!("{:?}", status as i32);


    let favorite = Color::Red;
    println!("{}", favorite);

    let blue = Color::Blue;
    println!("{}", blue);

    let pink = Color::Custom("#FFC0CB".to_string());
    println!("{}", pink);
}