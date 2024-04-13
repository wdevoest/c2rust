use std::io::{stdin, Read};

#[derive(Debug)]
struct Student {
    name: String,
    grade: String,
}

fn main() {
    let mut stu = Student {
        grade: String::from("nil"),
        ..Default::default()
    };

    stdin().read_line(&mut stu.name).unwrap();
    stu.name = stu.name.trim().to_string();

    println!("Hi {}! Your grade is {}.", stu.name, stu.grade);
}