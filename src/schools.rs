use std::cell::RefCell;

#[derive(Debug)]
pub struct Student {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug)]
pub struct School {
    student: RefCell<Vec<Student>>,
}


impl School {
    pub fn created() -> School {
        School {
            student: RefCell::new(Vec::new()),
        }
    }
    pub fn add(&self, user: Student) {
        self.student.borrow_mut().push(user);
    }

    // pub fn get(&self) -> Vec<Student> {
    //     return self.student.into_inner()
    // }

    pub fn print(&self) {
        for student in self.student.borrow().iter() {
            println!("{:?}", student)
        }
    }
}
