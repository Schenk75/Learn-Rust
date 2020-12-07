use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !(nucleotide == 'A' || nucleotide == 'T' || nucleotide == 'C' || nucleotide == 'G') {
        return Err(nucleotide);
    }
    match nucleotide_counts(dna) {
        Ok(map) => {
            let count = map.get(&nucleotide);
            match count {
                Some(size) => {
                    return Ok(*size);
                },
                None => {
                    return Ok(0);
                },
            }
        },
        Err(error) => {
            return Err(error);
        },
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut dna_hash: HashMap<char, usize> = HashMap::new();
    dna_hash.insert('A', 0);
    dna_hash.insert('T', 0);
    dna_hash.insert('C', 0);
    dna_hash.insert('G', 0);
    for ch in dna.chars() {
        if ch == 'A' || ch == 'T' || ch == 'C' || ch == 'G' {
            let count = dna_hash.entry(ch).or_insert(0);
            *count += 1;
        } else {
            return Err(ch);
        }
     }
     Ok(dna_hash)
}
