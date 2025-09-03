use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    seq: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    seq: String,
}

const DNA_NUCLEOTIDES: [char; 4] = ['G', 'C', 'T', 'A'];
const RNA_NUCLEOTIDES: [char; 4] = ['G', 'C', 'U', 'A'];

fn validate_nucleotides(seq: &str, valid_nucleotides: &[char]) -> Result<(), usize> {
    for (idx, nucleotide) in seq.chars().enumerate() {
        if !valid_nucleotides.contains(&nucleotide) {
            return Err(idx);
        }
    }
    Ok(())
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        //        todo!(
        //            "Construct new Dna from '{dna}' string. If string contains invalid nucleotides return index of first invalid nucleotide"
        //        );
        //
        validate_nucleotides(dna, &DNA_NUCLEOTIDES)?;
        Ok(Dna {
            seq: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        // todo!("Transform Dna {self:?} into corresponding Rna");
        let dna_to_rna = HashMap::from([('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')]);
        let seq = self
            .seq
            .chars()
            .map(|ch| dna_to_rna.get(&ch).unwrap())
            .collect();
        Rna { seq }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        //todo!(
        //    "Construct new Rna from '{rna}' string. If string contains invalid nucleotides return index of first invalid nucleotide"
        //);
        validate_nucleotides(rna, &RNA_NUCLEOTIDES)?;
        Ok(Rna {
            seq: rna.to_string(),
        })
    }
}
