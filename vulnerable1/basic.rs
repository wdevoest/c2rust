fn main() {
    let mut stu = Student {
        name: String::new(),
        grade: String::from("nil"),
    };

    std::io::stdin().read_line(&mut stu.name).expect("Failed to read name.");

    println!("Hi {}! Your grade is {}.", stu.name, stu.grade);
}

struct Student {
    name: String,
    grade: String,
}
