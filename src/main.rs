mod db;

use std::{io::{stdin, stdout}, str::FromStr};

const COURSE_NAME: &str = "Rust Course";
const MAX_STUDENT: u8 = 2;

#[derive(Clone)]
struct Student {
    name: String,
    age: u8,
}

fn prompt_input<T: FromStr>(prompt: &str) -> Result<T, &'static str> {
    println!("{}: ", prompt);
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    input.trim().parse().map_err(|_| "Cannot parse input")
}

fn input_student() -> Result<Student, &'static str> {
    print!("#### Adding New Student ####\n");

    let student_name: String = prompt_input("Enter Student Name")?;
    // Check for minimum 3 character length for Student name
    if student_name.len() < 3 {
        println!(
            "Student name cannot be less than 3 characters. Record not added.\n Please try again"
        );
        return Err("Student's name too short");
    }

    let age = prompt_input("Age of the Student")?;

    Ok(Student {
        name: student_name.to_string(),
        age,
    })
}

fn main() {
    // Create a Vector to store students record
    let mut student_db = db::StudentDatabase::new();

    println!("###############################");
    println!("#  Welcome to {}", COURSE_NAME);
    println!("###############################");

    for i in 1usize.. {
        // Check for max number of students tha can be added.
        let db_length = u8::try_from(student_db.len()).ok();
        if db_length >= Some(MAX_STUDENT) {
            // Already added max students that can be enrolled. Display the contents of Student DB and Exit
            println!(
                "Sorry! Only {} students can be added to the course.\nYou have already added {} students in the Course.",
                MAX_STUDENT, db_length.unwrap()
            );

            break;
        }

        // Add student to course
        let student = if let Ok(student) = input_student() {
            student
        } else {
            continue;
        };

        student_db.add(student.name.clone(), student.age);

        println!(
            "#### Added student '{}' with student number {} to Course ####",
            student.name, i
        );

        println!("Press any key to Continue. Press q to Exit");
        let mut input = String::new();
        let _b1 = std::io::stdin().read_line(&mut input).unwrap();

        let exit_var = &input[0..input.len() - 1];

        if exit_var == "q" {
            break;
        }
    }
    println!("Exiting...");
    student_db.display_on(&mut stdout()).expect("Unexpected output failure");
}
