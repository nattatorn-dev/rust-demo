use std::cell::RefCell;

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Book {
    lists: RefCell<Vec<i32>>,
}

impl Book {
    fn created() -> Book {
        Book {
            lists: RefCell::new(Vec::new()),
        }
    }
    fn add(&self, value: i32) {
        self.lists.borrow_mut().push(value);
    }
    fn print(&self) {
        for item in self.lists.borrow().iter() {
            println!("{}", item)
        }
    }
}

#[derive(Debug)]
struct Student {
    first_name: String,
    last_name: String,
}

#[derive(Debug)]
struct School {
    student: RefCell<Vec<Student>>,
}

impl School {
    fn created() -> School {
        School {
            student: RefCell::new(Vec::new()),
        }
    }
    fn add(&self, user: Student) {
        self.student.borrow_mut().push(user);
    }

    fn print(&self) {
        for student in self.student.borrow().iter() {
            println!("{:?}", student)
        }
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);

    let user2 = build_user(String::from("admin@admin.com"), String::from("admin"));
    println!("{:?}", user2);

    let books = Book::created();
    books.add(1);
    books.add(4);
    books.add(99);
    books.print();

    let schools = School::created();
    let john = Student {
        first_name: String::from("john"),
        last_name: String::from("legend"),
    };
    schools.add(john);
    schools.print();
}
