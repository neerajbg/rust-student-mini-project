use std::io::stdin;

pub const COURSE_NAME: &str = "Rust Course";
const MAX_STUDENT: u8 = 3;

pub mod db;

fn main() {
    // Read existing sudents in the course from db file
    let mut student_db = match db::parse_db::read_db() {
        Ok(db_data) => db_data,
        Err(_err) => {
            println!("{}", _err);
            Vec::new()
        } // return a new Vector of students in case of error
    };

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
