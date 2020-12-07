use std::collections::HashMap;

pub struct School {
    grade_list: HashMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        let grade_list = HashMap::new();
        School {grade_list}
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.grade_list.entry(grade).or_insert(vec![]);
        students.push(String::from(student));
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut ret = vec![];
        for (key, _) in &self.grade_list {
            ret.push(*key);
        }
        ret.sort();
        ret
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let mut students = self.grade_list.get(&grade)?.clone();
        students.sort();
        Some(students)
    }
}
