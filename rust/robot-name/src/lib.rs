use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_SET: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        // todo!("Construct a new Robot struct.");
        let mut robot = Self {
            name: String::new(),
        };

        robot.reset_name();
        robot
    }

    pub fn name(&self) -> &str {
        // todo!("Return the reference to the robot's name.");
        &self.name
    }

    pub fn reset_name(&mut self) {
        //todo!("Assign a new unique name to the robot.");
        loop {
            const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            let starts: String = (0..2)
                .map(|_| {
                    let idx = rand::random_range(0..CHARS.len());
                    CHARS[idx] as char
                })
                .collect();
            let ends = rand::random_range(100..999);

            let mut name = String::new();
            name.push_str(&starts);
            name.push_str(&ends.to_string());

            let mut names_lock = GLOBAL_SET.lock().unwrap();
            if !names_lock.contains(&name) {
                self.name = name.clone();
                names_lock.insert(name);

                println!("{:?}", names_lock);

                break;
            }
        }
    }
}
