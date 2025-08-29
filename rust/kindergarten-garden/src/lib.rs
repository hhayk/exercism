use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    //    todo!("based on the {diagram}, determine the plants the {student} is responsible for");
    let names = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let plants = HashMap::from([
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]);

    let rows: Vec<&str> = diagram.split('\n').collect();
    let (row1, row2) = (rows[0], rows[1]);

    println!("{:?}, {:?}", row1, row2);

    if let Some(idx) = names.iter().position(|&n| n == student) {
        let mut ret: Vec<&str> = Vec::new();
        let index = idx * 2;

        let str1 = &row1[index..=index + 1];
        let str2 = &row2[index..=index + 1];
        let str = format!("{}{}", str1, str2);

        for ch in str.chars() {
            ret.push(plants.get(&ch).unwrap());
        }

        return ret;
    }

    Vec::new()
}
