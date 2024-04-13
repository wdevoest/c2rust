use std::io::{self, Write};

#[derive(Debug)]
struct Student {
    name: String,
    grade: String,
}

fn main() {
    let mut student = Student {
        name: String::new(),
        grade: "nil".to_string(),
    };

    print!("Enter your name: ");
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(&mut student.name).expect("Could not read line");

    println!("Hi {}! Your grade is {}.", student.name, student.grade);
}