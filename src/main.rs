use std::io::{stdin, BufRead, BufReader};

pub const COURSE_NAME: &str = "Rust Course";
const MAX_STUDENT: u8 = 3;

pub mod db;

// Read from db file
fn read_db() -> Vec<db::Student> {
    let file_name = "db.db";

    let file = match std::fs::File::open(file_name) {
        Ok(file) => file,
        Err(error) => {
            panic!("Error reading file. {}", error);
        }
    };

    // Vector to hold students record
    let mut student_db: Vec<db::Student> = Vec::new();

    // Create a Reader
    let reader = BufReader::new(file);

    // Iterate through the file and convert file string into vector of student
    for line in reader.lines() {
        let line = line.expect("Error in reading line");

        // Split with tabs
        let tabbed_lines = line.split("\t");

        let collection = tabbed_lines.collect::<Vec<&str>>();

        // Split with ":"

        // Variables to hold name and Age of existing students
        let mut name = String::new();
        let mut age = String::new();

        for item in collection {
            let temp_line = item.split(":");
            let data = temp_line.collect::<Vec<&str>>();

            // println!("{}\n", st1[0]);

            if data[0] == "name" {
                name = data[1].to_string();
            } else if data[0] == "age" {
                age = data[1].to_string();
            }
        }

        let age1 = age.parse::<u8>().unwrap();

        let temp_st = db::Student::new(name, age1);
        student_db.push(temp_st);
    }

    // Return students vector
    student_db
}

fn main() {
    // Create a Vector to store students record
    // let mut student_db: Vec<db::Student> = Vec::new();

    // Read existing sudents in the course
    let mut student_db = read_db();

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
            db::display_students_in_course(&student_db);
            break;
        }

        // Add student to course

        let Some(st )= db::add_student() else {
            continue;
        };

        // Add new student to student DB
        student_db.push(st.clone());

        println!(
            "#### Added student '{}' with student number {} to Course ####",
            st.name, i
        );

        println!("Press any key to Continue. Press q to Exit");
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_n) => {}
            Err(_error) => println!("Something went wrong!"),
        }

        let exit_var = &input[0..input.len() - 1];

        if exit_var == "q" {
            println!("Exiting...");
            db::display_students_in_course(&student_db);
            break;
        }
        i += 1;
    }
}
