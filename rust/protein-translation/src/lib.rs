pub fn translate(rna: &str) -> Option<Vec<&str>> {
    //    todo!(
    //        "Return a list of protein names that correspond to the '{rna}' RNA string or None if the RNA string is invalid"
    //    );
    let mut vec = Vec::new();

    for slice in rna.as_bytes().chunks(3) {
        if let Ok(ss) = String::from_utf8(slice.to_vec()) {
            let res = match ss.as_str() {
                "AUG" => "Methionine",
                "UUU" | "UUC" => "Phenylalanine",
                "UUA" | "UUG" => "Leucine",
                "UCU" | "UCC" | "UCA" | "UCG" => "Serine",
                "UAU" | "UAC" => "Tyrosine",
                "UGU" | "UGC" => "Cysteine",
                "UGG" => "Tryptophan",
                "UAA" | "UAG" | "UGA" => break,
                _ => return None,
            };

            vec.push(res);
        }
    }

    Some(vec)
}
