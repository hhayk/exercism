use std::collections::HashMap;

const VALID_NUCLEOIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // todo!("How much of nucleotide type '{nucleotide}' is contained inside DNA string '{dna}'?");
    match dna.chars().find(|c| !VALID_NUCLEOIDES.contains(c)) {
        Some(ch) => Err(ch),
        None => {
            if VALID_NUCLEOIDES.contains(&nucleotide) {
                Ok(dna.chars().filter(|&ch| ch == nucleotide).count())
            } else {
                Err(nucleotide)
            }
        }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    // todo!("How much of every nucleotide type is contained inside DNA string '{dna}'?");
    match dna.chars().find(|c| !VALID_NUCLEOIDES.contains(c)) {
        Some(ch) => Err(ch),
        None => {
            let map = dna.chars().fold(
                HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]),
                |mut acc, ch| {
                    acc.entry(ch).and_modify(|count| *count += 1).or_insert(0);
                    acc
                },
            );

            Ok(map)
        }
    }
}
