use std::io::stdin;

fn main() {
    const COURSE_NAME: &str = "Rust Course";
    const MAX_STUDENT: i8 = 2;
    let mut student_db: Vec<Student> = Vec::new();

    struct Student {
        name: String,
        age: u8,
    }

    println!("###############################");
    println!("#  Welcome to {}", COURSE_NAME);
    println!("###############################");

    let mut i: i8 = 1;
    while true {
        print!("#### Adding New Student ####\n");
        println!("Enter Student Name: ");

        let mut input = String::new();
        let _b1 = stdin().read_line(&mut input);

        // TODO Check for max number of students tha can be added.

        // Check for minimum 3 character length for Student name
        let student_name = &input[..input.len() - 1].trim();
        if student_name.len() < 3 {
            println!(
                "Student name cannot be less than 3 haracter. Record not added.\n Please try again"
            );
            continue;
        }

        // Ask for age of the student
        println!("Age of the Student");
        let mut input = String::new();

        stdin().read_line(&mut input);

        let age = input.trim();
        age.to_string().pop(); // Remove newline character
        let st = Student {
            name: student_name.to_string(),
            age: age.parse().unwrap(),
        };

        student_db.push(st);

        println!("Length of student: {}", student_name.len());
        println!(
            "#### Added student '{}' with student number {} to Course ####",
            student_name, i
        );

        println!("Press any key to Continue. Press q to Exit");
        let mut input = String::new();
        let _b1 = std::io::stdin().read_line(&mut input).unwrap();

        let exit_var = &input[0..input.len() - 1];

        if exit_var == "q" {
            println!("Exiting...");
            println!("\n\n\n\nStudents added in this course\n");
            for i in student_db {
                println!("Name: {}, Age: {}", i.name, i.age)
            }
            break;
        }
        i += 1;
    }
}
