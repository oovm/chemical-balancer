use crate::Atom;
use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{Debug, Formatter},
};

impl Debug for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.count.partial_cmp(&1.0) {
            Some(Ordering::Equal) => write!(f, "{}", self.atom),
            _ => write!(f, "{}{}", self.atom, self.count),
        }
    }
}

impl Atom {
    pub fn record_elements(&self, all: &mut BTreeSet<String>) {
        all.insert(self.atom.clone());
    }
    pub fn count_elements(&self, all: &BTreeSet<String>, count: &mut Vec<f64>) {
        match all.iter().position(|x| self.atom.eq(x)) {
            Some(i) => {
                count[i] += self.count;
            }
            None => {}
        }
    }
}
