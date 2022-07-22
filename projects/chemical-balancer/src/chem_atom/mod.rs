use crate::{Atom, ChemicalBalancer, ChemicalKind, ChemicalTerm};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Formatter},
};
mod display;
mod solver;
use num::{Integer, One};
use rationalize::float2ratio;
impl ChemicalBalancer {
    pub fn count_elements(&self, compound: &ChemicalTerm) -> Vec<f64> {
        compound.count_elements(&self.elements)
    }
    pub fn get_elements(&self) -> &BTreeSet<String> {
        &self.elements
    }
    pub fn record_elements(&mut self) {
        for i in &self.lhs {
            i.record_elements(&mut self.elements);
        }
        for i in &self.rhs {
            i.record_elements(&mut self.elements);
        }
    }
}

impl Debug for ChemicalTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChemicalTerm::Atom(v) => Debug::fmt(v, f),
            ChemicalTerm::Compound(v) => Debug::fmt(v, f),
        }
    }
}

impl ChemicalTerm {
    pub fn record_elements(&self, all: &mut BTreeSet<String>) {
        match self {
            ChemicalTerm::Atom(atom) => {
                atom.record_elements(all);
            }
            ChemicalTerm::Compound(compound) => {
                compound.record_elements(all);
            }
        }
    }
    pub fn count_elements(&self, all: &BTreeSet<String>) -> Vec<f64> {
        let mut out = vec![0.0; all.len()];
        match self {
            ChemicalTerm::Atom(atom) => {
                atom.count_elements(all, &mut out);
                out
            }
            ChemicalTerm::Compound(compound) => compound.count_elements(all),
        }
    }
}
