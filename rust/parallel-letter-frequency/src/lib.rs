use std::{collections::HashMap, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut threads = Vec::with_capacity(worker_count);

    let input = input.join("");
    let mut chars = input.chars();
    let chunk_length = input.len() / worker_count + 1;

    let thunk = |chunk: String| {
        let mut map = HashMap::new();
        for ch in chunk.chars() {
            if ch.is_alphabetic() {
                map.entry(ch.to_ascii_lowercase())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
        map
    };

    for _ in 0..worker_count {
        let chunk = chars.by_ref().take(chunk_length).collect::<String>();
        let thread = thread::spawn(move || thunk(chunk));

        threads.push(thread);
    }

    let mut ret = HashMap::new();
    for thread in threads {
        let res = thread.join().unwrap();
        for (key, value) in res {
            *ret.entry(key).or_default() += value;
        }
    }
    ret
}
