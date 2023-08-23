use std::io::{stdin, Write};

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

    fn tabbed_record(&self) -> String {
        format!("name:{}\tage:{}\n", self.name, self.age)
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

    // Save to db.db file
    save_to_file(st_db);
}

// Function to save student db in file
fn save_to_file(st_db: &[Student]) {
    let mut file_conent = String::new();

    for item in st_db.iter() {
        file_conent.push_str(item.tabbed_record().as_str());
    }

    let mut file = std::fs::File::create("db.db").expect("Could'nt create the file");

    file.write_all(file_conent.as_bytes())
        .expect("Error in writting to file");
}
