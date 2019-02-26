use std::collections::HashMap;

pub struct Monoisotopic {
    table: HashMap<char, f64>,
}

impl Default for Monoisotopic {
    fn default() -> Self {
        let mut table = HashMap::new();
        table.insert('A', 71.03711);
        table.insert('C', 103.00919);
        table.insert('D', 115.02694);
        table.insert('E', 129.04259);
        table.insert('F', 147.06841);
        table.insert('G', 57.02146);
        table.insert('H', 137.05891);
        table.insert('I', 113.08406);
        table.insert('K', 128.09496);
        table.insert('L', 113.08406);
        table.insert('M', 131.04049);
        table.insert('N', 114.04293);
        table.insert('P', 97.05276);
        table.insert('Q', 128.05858);
        table.insert('R', 156.10111);
        table.insert('S', 87.03203);
        table.insert('T', 101.04768);
        table.insert('V', 99.06841);
        table.insert('W', 186.07931);
        table.insert('Y', 163.06333);
        Monoisotopic { table }
    }
}

impl Monoisotopic {
    pub fn get(&self, ch: char) -> Option<f64> {
        self.table.get(&ch).cloned()
    }

    pub fn peptide_mass(&self, s: &str) -> f64 {
        s.chars()
            .filter_map(|ch| self.get(ch))
            .fold(0f64, |f, x| f + x)
    }
}
