use std::cell::RefCell;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

pub fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
pub struct Book {
    lists: RefCell<Vec<i32>>,
}

impl Book {
    pub fn created() -> Book {
        Book {
            lists: RefCell::new(Vec::new()),
        }
    }
    pub fn add(&self, value: i32) {
        self.lists.borrow_mut().push(value);
    }
    pub fn print(&self) {
        for item in self.lists.borrow().iter() {
            println!("{}", item)
        }
    }
}
