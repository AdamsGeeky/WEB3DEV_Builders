mod student;
use std::io::{self, Write};
use student::{Student, STUDENT_LIST};

fn main() {
    for _ in 0..2 {
        // Prompt twice for demonstration
        let student = create_student_from_input();
        println!("Student added: {}", student);
    }

    println!("\nAll Students:");
    let students = STUDENT_LIST.lock().unwrap();
    for student in students.iter() {
        println!("{}", student);
    }
}

fn create_student_from_input() -> Student {
    let name = prompt("Enter the student's name: ");
    let age: u8 = prompt("Enter the student's age: ")
        .trim()
        .parse()
        .expect("Please enter a valid age");
    let marks: u8 = prompt("Enter the student's marks: ")
        .trim()
        .parse()
        .expect("Please enter valid marks");
    let grade = prompt("Enter the student's grade: ");

    Student::new(name, age, marks, grade)
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}
