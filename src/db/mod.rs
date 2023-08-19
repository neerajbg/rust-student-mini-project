use std::io::stdin;

use crate::COURSE_NAME;

#[derive(Clone, Default)]
pub struct Student {
    pub name: String,
    age: u8,
}

impl Student {
    // Public Constructor function to create a new instance of Student
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    // Private function to print student profile
    fn print_profile(&self) {
        println!("Name: {}, Age: {}", self.name, self.age)
    }
}

pub fn add_student() -> Option<Student> {
    print!("#### Adding New Student ####\n");
    println!("Enter Student Name: ");

    // Take user input for Student name
    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_n) => {}
        Err(_error) => println!("Something went wrong!"),
    };

    let student_name = &input[..input.len() - 1].trim();

    // Check for minimum 3 character length for Student name
    if student_name.len() < 3 {
        println!(
            "Student name cannot be less than 3 characters. Record not added.\n Please try again"
        );

        return None;
    }

    // Take user input for Student Age
    println!("Age of the Student");
    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_n) => {}
        Err(_error) => println!("Something went wrong!"),
    };

    let age = input.trim().parse().unwrap_or_default();

    // Create  object of student struct and return
    Some(Student::new(student_name.to_string(), age))
}

// Function to display students already enrolled in the course
pub fn display_students_in_course(st_db: &[Student]) {
    let db_label = format!("\n# Students added in {} #", COURSE_NAME);

    println!();
    for _ in 1..db_label.len() {
        print!("#");
    }
    println!("{}", db_label);
    for _ in 1..db_label.len() {
        print!("#");
    }
    println!("\n");

    for item in st_db.iter() {
        item.print_profile();
    }
}
