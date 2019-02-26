use std::collections::HashMap;

pub struct CodonTable {
    map: HashMap<&'static str, char>,
}

impl Default for CodonTable {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("UUU", 'F');
        map.insert("CUU", 'L');
        map.insert("AUU", 'I');
        map.insert("GUU", 'V');
        map.insert("UUC", 'F');
        map.insert("CUC", 'L');
        map.insert("AUC", 'I');
        map.insert("GUC", 'V');
        map.insert("UUA", 'L');
        map.insert("CUA", 'L');
        map.insert("AUA", 'I');
        map.insert("GUA", 'V');
        map.insert("UUG", 'L');
        map.insert("CUG", 'L');
        map.insert("AUG", 'M');
        map.insert("GUG", 'V');
        map.insert("UCU", 'S');
        map.insert("CCU", 'P');
        map.insert("ACU", 'T');
        map.insert("GCU", 'A');
        map.insert("UCC", 'S');
        map.insert("CCC", 'P');
        map.insert("ACC", 'T');
        map.insert("GCC", 'A');
        map.insert("UCA", 'S');
        map.insert("CCA", 'P');
        map.insert("ACA", 'T');
        map.insert("GCA", 'A');
        map.insert("UCG", 'S');
        map.insert("CCG", 'P');
        map.insert("ACG", 'T');
        map.insert("GCG", 'A');
        map.insert("UAU", 'Y');
        map.insert("CAU", 'H');
        map.insert("AAU", 'N');
        map.insert("GAU", 'D');
        map.insert("UAC", 'Y');
        map.insert("CAC", 'H');
        map.insert("AAC", 'N');
        map.insert("GAC", 'D');
        map.insert("UAA", 'X');
        map.insert("CAA", 'Q');
        map.insert("AAA", 'K');
        map.insert("GAA", 'E');
        map.insert("UAG", 'X');
        map.insert("CAG", 'Q');
        map.insert("AAG", 'K');
        map.insert("GAG", 'E');
        map.insert("UGU", 'C');
        map.insert("CGU", 'R');
        map.insert("AGU", 'S');
        map.insert("GGU", 'G');
        map.insert("UGC", 'C');
        map.insert("CGC", 'R');
        map.insert("AGC", 'S');
        map.insert("GGC", 'G');
        map.insert("UGA", 'X');
        map.insert("CGA", 'R');
        map.insert("AGA", 'R');
        map.insert("GGA", 'G');
        map.insert("UGG", 'W');
        map.insert("CGG", 'R');
        map.insert("AGG", 'R');
        map.insert("GGG", 'G');

        CodonTable { map }
    }
}

impl CodonTable {
    pub fn translate(&self, input: &str) -> Option<String> {
        let mut output = String::new();
        for i in (0..input.len() - 3).step_by(3) {
            let s = &input[i..i + 3];
            match self.get(s) {
                None => return None,
                Some('X') => break,
                Some(c) => output.push(c),
            }
        }
        Some(output)
    }

    pub fn get(&self, s: &str) -> Option<char> {
        self.map.get(s).cloned()
    }
}
