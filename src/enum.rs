use std::fmt;


#[allow(dead_code)]
#[derive(Debug)]
enum Status {
    Online,
    Offline,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Orange,
    Custom(String),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Color::Red => write!(f, "#FF0000"),
            Color::Green => write!(f, "#008000"),
            Color::Blue => write!(f, "#0000FF"),
            Color::Orange => write!(f, "#FFA500"),
            Color::Custom(color) => write!(f, "{}", color),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

#[allow(dead_code)]
#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Shape {
    Rectangle { width: u32, height: u32},
    Square(u32),
    Circle(f64)
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle { width, height} => (width * height) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14 * (r * r),
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

    let up = Direction::Up(Point { x: 0, y: 1 });
    let up_direction = up.match_direction();
    println!("{:?}", up_direction);

    let key = up_direction.destruct();
    println!("{:?}", key);


    let rectangle = Shape::Rectangle{width: 10, height: 70};
    let square = Shape::Square(10);
    let circle = Shape::Circle(4.5);

    let rectangle_area = rectangle.area();
    println!("{}", rectangle_area);

    let square_area = square.area();
    println!("{}", square_area);

    let circle_area = circle.area();
    println!("{}", circle_area);
}
