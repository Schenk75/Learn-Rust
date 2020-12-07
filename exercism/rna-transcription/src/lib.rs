use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let err_idx = dna.find(|x| {x != 'A' && x != 'C' && x != 'T' && x != 'G'});
        match err_idx {
            Some(i) => {
                return Err(i);
            },
            None => {
                return Ok(Dna(String::from(dna)));
            }
        }
    }

    pub fn into_rna(self) -> Rna {
        let mut map = HashMap::new();
        map.insert('A', 'U');
        map.insert('C', 'G');
        map.insert('T', 'A');
        map.insert('G', 'C');

        let mut ret = String::new();
        for ch in self.0.chars() {
            ret.push(*map.get(&ch).unwrap());
        }

        Rna(ret)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let err_idx = rna.find(|x| {x != 'A' && x != 'C' && x != 'U' && x != 'G'});
        match err_idx {
            Some(i) => {
                return Err(i);
            },
            None => {
                return Ok(Rna(String::from(rna)));
            }
        }
    }
}
