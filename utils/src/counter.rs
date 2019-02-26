//! Convenience wrapper around a HashMap<T, usize> for counting elements in a collection
#![allow(dead_code)]
use std::collections::HashMap;
use std::iter::FromIterator;

pub struct Counter<T: Eq + std::hash::Hash> {
    map: HashMap<T, usize>,
}

impl<T: Eq + std::hash::Hash> Default for Counter<T> {
    fn default() -> Counter<T> {
        Counter {
            map: HashMap::new(),
        }
    }
}

impl<T: Eq + std::hash::Hash> Counter<T> {
    pub fn add(&mut self, t: T) {
        *self.map.entry(t).or_insert(0) += 1;
    }

    pub fn count(&mut self, t: &T) -> Option<usize> {
        self.map.get(t).cloned()
    }

    pub fn most(&self) -> Option<&T> {
        let mut n = 0;
        let mut ptr = None;
        for (k, &v) in &self.map {
            if v >= n {
                ptr = Some(k);
                n = v;
            }
        }
        ptr
    }

    pub fn least(&self) -> Option<&T> {
        let mut n = std::usize::MAX;
        let mut ptr = None;
        for (k, &v) in &self.map {
            if v <= n {
                ptr = Some(k);
                n = v;
            }
        }
        ptr
    }
}

impl<T: Eq + std::hash::Hash> FromIterator<T> for Counter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = Counter::default();
        for i in iter {
            c.add(i);
        }
        c
    }
}

impl<T: Eq + std::hash::Hash> Extend<T> for Counter<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter {
            self.add(i);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let mut c = Counter::new();
        c.add(1);
        c.add(2);
        c.add(1);
        assert_eq!(c.count(&1), Some(2));
        assert_eq!(c.most(), Some(&1));
        assert_eq!(c.least(), Some(&2));
    }

    #[test]
    fn nucleotides() {
        use super::super::{Nucleotide, Sequence};
        let s = "ATCTGGTCCAATCTTT";

        let seq = Sequence::new(s).nucleotides();

        let c = seq.into_iter().collect::<Counter<Nucleotide>>();
        assert_eq!(c.least(), Some(&Nucleotide::G));
        assert_eq!(c.most(), Some(&Nucleotide::T));
    }
}
