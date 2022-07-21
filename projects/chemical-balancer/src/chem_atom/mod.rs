use crate::{Atom, ChemicalBalancer, ChemicalTerm, CompoundGroup};
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
            ChemicalTerm::Atom(atom) => Debug::fmt(atom, f),
            ChemicalTerm::Compound { compound, count, .. } => {
                let mut v = &mut f.debug_tuple("Compound");
                for item in compound {
                    v = v.field(item);
                }
                v = v.field(count);
                v.finish()
            }
        }
    }
}

impl ChemicalTerm {
    pub fn record_elements(&self, all: &mut BTreeSet<String>) {
        match self {
            ChemicalTerm::Atom(atom) => {
                atom.record_elements(all);
            }
            ChemicalTerm::Compound { compound, .. } => {
                for c in compound {
                    c.record_elements(all);
                }
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
            ChemicalTerm::Compound { compound, count, .. } => {
                let mut out = vec![0.0; all.len()];
                for c in compound {
                    let sub_out = c.count_elements(all);
                    for (i, v) in sub_out.iter().enumerate() {
                        out[i] += v * count;
                    }
                }
                out
            }
        }
    }
}

impl ChemicalTerm {
    pub fn atom(atom: String, count: f64) -> Self {
        ChemicalTerm::Atom(Atom { atom, count, electronic: 0.0 })
    }

    pub fn compound(compound: Vec<ChemicalTerm>, count: f64) -> Self {
        ChemicalTerm::Compound { group: CompoundGroup::None, compound, count, electronic: 0.0 }
    }
    pub fn parentheses(compound: Vec<ChemicalTerm>, count: f64) -> Self {
        ChemicalTerm::Compound { group: CompoundGroup::Parentheses, compound, count, electronic: 0.0 }
    }
    pub fn with_electronic(mut self, e: f64) -> Self {
        match &mut self {
            ChemicalTerm::Atom(atom) => {
                atom.electronic = e;
            }
            ChemicalTerm::Compound { electronic, .. } => {
                *electronic = e;
            }
        }
        self
    }
}
