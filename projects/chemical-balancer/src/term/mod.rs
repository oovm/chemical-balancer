use crate::{ChemicalKind, ChemicalTerm, Compound};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Formatter},
};

impl Debug for Compound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = &mut f.debug_tuple("Compound");
        for item in &self.compound {
            v = v.field(item);
        }
        v = v.field(&self.count);
        v.finish()
    }
}

impl ChemicalTerm {
    pub fn atom<S>(atom: S) -> Self
    where
        S: Into<String>,
    {
        ChemicalTerm { kind: ChemicalKind::Atomic(atom.into()), compound: vec![], count: 1.0, electronic: 0.0 }
    }
    pub fn parentheses(compound: Vec<ChemicalTerm>) -> Self {
        ChemicalTerm { kind: ChemicalKind::Paired('(', ')'), compound, count: 1.0, electronic: 0.0 }
    }
    pub fn brackets(compound: Vec<ChemicalTerm>) -> Self {
        ChemicalTerm { kind: ChemicalKind::Paired('[', ']'), compound, count: 1.0, electronic: 0.0 }
    }
    pub fn curly(compound: Vec<ChemicalTerm>) -> Self {
        ChemicalTerm { kind: ChemicalKind::Paired('{', '}'), compound, count: 1.0, electronic: 0.0 }
    }
    pub fn is_atom(&self) -> bool {
        self.compound.is_empty()
    }
    pub fn is_compound(&self) -> bool {
        !self.is_atom()
    }
    pub fn with_number(mut self, count: f64) -> Self {
        self.count = count;
        self
    }
    pub fn with_electronic(mut self, e: f64) -> Self {
        self.electronic = e;
        self
    }
}

impl ChemicalTerm {
    pub fn record_elements(&self, all: &mut BTreeSet<String>) {
        for c in &self.compound {
            c.record_elements(all);
        }
    }
    pub fn count_elements(&self, all: &BTreeSet<String>) -> Vec<f64> {
        let mut out = vec![0.0; all.len()];
        for c in &self.compound {
            let sub_out = c.count_elements(all);
            for (i, v) in sub_out.iter().enumerate() {
                out[i] += v * self.count;
            }
        }
        out
    }
}
