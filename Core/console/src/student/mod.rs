use lazy_static::lazy_static;
use std::sync::Mutex;

pub struct Student {
    pub(crate) name: String,
    pub(crate) age: u8,
    pub(crate) marks: u8,
    pub(crate) grade: String,
}

impl Student {
    pub fn new(name: String, age: u8, marks: u8, grade: String) -> Student {
        let student = Student {
            name,
            age,
            marks,
            grade,
        };
        STUDENT_LIST.lock().unwrap().push(student.clone());
        student
    }
}

impl Clone for Student {
    fn clone(&self) -> Student {
        Student {
            name: self.name.clone(),
            age: self.age,
            marks: self.marks,
            grade: self.grade.clone(),
        }
    }mod models {
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}, Age: {}, Marks: {}, Grade: {}",
            self.name, self.age, self.marks, self.grade
        )
    }
}

lazy_static! {
    pub static ref STUDENT_LIST: Mutex<Vec<Student>> = Mutex::new(Vec::new());
}
