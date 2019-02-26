use std::fmt;

pub struct Sequence<'s> {
    slice: &'s str,
}

impl<'s> Sequence<'s> {
    pub fn new(s: &'s str) -> Sequence<'s> {
        Sequence { slice: s }
    }

    pub fn nucleotides(&self) -> Vec<Nucleotide> {
        self.slice.chars().map(Nucleotide::from_char).collect()
    }

    pub fn reverse_complement(&self) -> Vec<Nucleotide> {
        self.slice
            .chars()
            .rev()
            .map(Nucleotide::from_char)
            .map(Nucleotide::complement)
            .collect()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Nucleotide {
    A = 0,
    C = 1,
    G = 2,
    T = 3,
    U = 4,
}

impl Nucleotide {
    /// Convert from nucleotide to UTF8 character
    pub fn to_char(self) -> char {
        match self {
            Nucleotide::A => 'A',
            Nucleotide::C => 'C',
            Nucleotide::G => 'G',
            Nucleotide::T => 'T',
            Nucleotide::U => 'U',
        }
    }

    /// Convert from a capital letter to nucleotide
    ///
    /// # Panic
    ///
    /// Will panic if `ch` is not 'A', 'C', 'G', 'T', or 'U'
    pub fn from_char(ch: char) -> Nucleotide {
        match ch {
            'A' => Nucleotide::A,
            'C' => Nucleotide::C,
            'G' => Nucleotide::G,
            'T' => Nucleotide::T,
            'U' => Nucleotide::U,
            _ => panic!("Invalid nucleotide: character `{}`", ch),
        }
    }

    /// Transcription of DNA to RNA
    pub fn transcribe(self) -> Nucleotide {
        match self {
            Nucleotide::T => Nucleotide::U,
            _ => self,
        }
    }

    pub fn complement(self) -> Nucleotide {
        match self {
            Nucleotide::A => Nucleotide::T,
            Nucleotide::C => Nucleotide::G,
            Nucleotide::G => Nucleotide::C,
            Nucleotide::T => Nucleotide::A,
            Nucleotide::U => Nucleotide::A,
        }
    }
}

impl fmt::Display for Nucleotide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn revcomp() {
        let fwd = "ATATATAAACCG";
        let rev = "CGGTTTATATAT";
        assert_eq!(
            Sequence::new(fwd).reverse_complement(),
            Sequence::new(rev).nucleotides()
        );
    }
}
