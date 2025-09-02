use std::collections::HashMap;

pub struct School {
    hash: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School {
            hash: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        // todo!("Add {student} to the roster for {grade}")
        if !self.hash.contains_key(student) {
            self.hash.insert(student.to_string(), grade);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut vec: Vec<u32> = self.hash.values().copied().collect();
        vec.dedup();
        vec.sort();
        vec
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        // todo!("Return the list of students in {grade}")
        let mut ret: Vec<String> = self
            .hash
            .iter()
            .filter(|&(_, &hg)| hg == grade)
            .map(|(name, _)| name.clone())
            .collect();

        ret.sort();
        ret
    }
}
