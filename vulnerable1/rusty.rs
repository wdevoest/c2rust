use std::io;

struct Student {
    name: String,
    grade: String,
}

fn main() {
    let mut student = Student {
        name: String::new(),
        grade: "nil".to_string(),
    };

    println!("Enter student name:");
    io::stdin().read_line(&mut student.name).unwrap();
    student.name = student.name.trim().to_string(); // Remove trailing newline

    println!("Hi {}! Your grade is {}.", student.name, student.grade);
}