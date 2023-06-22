use std::io;
use super::Student;

pub struct StudentDatabase {
    db: Vec<Student>
}

impl StudentDatabase {
    pub fn new() -> Self {
        Self {
            db: vec![]
        }
    }

    pub fn add(&mut self, name: String, age: u8) {
        self.db.push(Student {
            name,
            age
        })
    }

    pub fn display_on(&self, output: &mut impl std::io::Write) -> io::Result<()> {
        for student in self.db.as_slice() {
            write!(output, "Name: {}, Age: {}\n", student.name, student.age)?;    
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.db.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_to_database() {
        let mut db_ut = StudentDatabase::new();
        db_ut.add("Test Student".to_string(), 34);
        assert_eq!(db_ut.len(), 1);
    }

    #[test]
    fn print_database() {
        let mut db_ut = StudentDatabase::new();
        db_ut.add("Test Student".to_string(), 34);
        db_ut.add("Foo Bar".to_string(), 43);
        
        let mut output: Vec<u8> = Vec::new();
        db_ut.display_on(&mut output).expect("Unexpected output failure");

        let result = String::from_utf8_lossy(&output);
        assert_eq!(result, "Name: Test Student, Age: 34\nName: Foo Bar, Age: 43\n");
    }
}