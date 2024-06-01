mod models {
    pub mod student;
}

use crate::models::student::{Student, STUDENT_LIST};

fn main() {
    let _student1 = Student::new(String::from("Adam Muhammad"), 20, 85, String::from("A"));
    let _student2 = Student::new(String::from("ishfad Geek"), 17, 90, String::from("A+"));

    // Accept user input

    print!("*** Student List **\n");
    let students = STUDENT_LIST.lock().unwrap();
    for student in students.iter() {
        println!("{}", student);
    }
}
