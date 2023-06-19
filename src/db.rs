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

    pub fn display(&self) {
        for student in self.db.as_slice() {
            println!("Name: {}, Age: {}", student.name, student.age);
        }
    }

    pub fn len(&self) -> usize {
        self.db.len()
    }
}
