// pub mod db;

use std::io::{BufRead, BufReader};

pub fn read_db() -> Result<Vec<super::Student>, &'static str> {
    let file_name = "db.db";

    let file = match std::fs::File::open(file_name) {
        Ok(file) => file,
        Err(_error) => {
            return Err("Error opening file.\n");
        }
    };

    // Vector to hold students record
    let mut student_db: Vec<super::Student> = Vec::new();

    // Create a Reader
    let reader = BufReader::new(file);

    // Iterate through the file and convert file string into vector of student
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_err) => {
                return Err("Error reading file.");
            }
        };

        // let temp_st =
        match parse_file_content(&line) {
            Ok(st) => {
                student_db.push(st);
            }
            Err(_err) => println!("Couldnt create Struct."),
        };
    }

    // Return students vector
    Ok(student_db)
}

fn parse_file_content(line: &String) -> Result<super::Student, &str> {
    // Split with tabs
    let tabbed_lines = line.split("\t");

    let collection = tabbed_lines.collect::<Vec<&str>>();

    // Variables to hold name and Age of existing students
    let mut name = String::new();
    let mut age = String::new();

    for item in collection {
        let temp_line = item.split(":");
        let data = temp_line.collect::<Vec<&str>>();

        // println!("{}\n", st1[0]);

        match data[0] {
            "name" => name = data[1].to_string(),
            "age" => age = data[1].to_string(),
            _ => println!("Dont know what it is"),
        };
    }

    let age1 = age.parse::<u8>().unwrap();

    let temp_st = super::Student::new(name, age1);

    Ok(temp_st)
}
