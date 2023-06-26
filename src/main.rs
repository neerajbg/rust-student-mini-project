use std::io::stdin;

const COURSE_NAME: &str = "Rust Course";
const MAX_STUDENT: u8 = 2;

#[derive(Clone)]
struct Student {
    name: String,
    age: u8,
}

// Function to add a new student to DB
fn add_student() -> Result<Student, bool> {
    print!("#### Adding New Student ####\n");
    println!("Enter Student Name: ");

    // Create empty object of student struct
    let mut st = Student {
        name: "".to_string(),
        age: 0,
    };

    // Take user input for Student name
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);

    let student_name = &input[..input.len() - 1].trim();

    // Check for minimum 3 character length for Student name
    if student_name.len() < 3 {
        println!(
            "Student name cannot be less than 3 characters. Record not added.\n Please try again"
        );

        return Err(false);
    }

    // Take user input for Student Age
    println!("Age of the Student");
    let mut input = String::new();

    let _ = stdin().read_line(&mut input);

    let age = input.trim();

    st.name = student_name.to_string();
    st.age = age.parse().unwrap();

    Ok(st)
}

// Function to display students already enrolled in the course
fn display_students_in_course(st_db: &[Student]) {
    println!("\n\nStudents added in this course\n");
    for i in st_db {
        println!("Name: {}, Age: {}", i.name, i.age)
    }
}

fn main() {
    // Create a Vector to store students record
    let mut student_db: Vec<Student> = Vec::new();

    println!("###############################");
    println!("#  Welcome to {}", COURSE_NAME);
    println!("###############################");

    let mut i: i8 = 1;
    loop {
        // Check for max number of students tha can be added.
        let db_length = student_db.len() as u8;
        if db_length >= MAX_STUDENT {
            // Already added max students that can be enrolled. Display the contents of Student DB and Exit
            println!(
                "Sorry! Only {} students can be added to the course.\nYou have already added {} students in the Course.",
                MAX_STUDENT, db_length
            );

            // Display students DB
            display_students_in_course(&student_db);
            break;
        }

        // Add student to course

        // Check for error. If error, continue the loop
        let st = match add_student() {
            Err(_) => continue,
            Ok(st) => st, // Ok(st) => st;
        };

        // Add new student to student DB
        student_db.push(st.clone());

        println!(
            "#### Added student '{}' with student number {} to Course ####",
            st.name, i
        );

        println!("Press any key to Continue. Press q to Exit");
        let mut input = String::new();
        let _b1 = std::io::stdin().read_line(&mut input).unwrap();

        let exit_var = &input[0..input.len() - 1];

        if exit_var == "q" {
            println!("Exiting...");
            display_students_in_course(&student_db);
            break;
        }
        i += 1;
    }
}
