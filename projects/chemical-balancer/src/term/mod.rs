use crate::{ChemicalBalancer, ChemicalKind, ChemicalTerm};
use num::Zero;
use std::{
    collections::BTreeSet,
    fmt::{Debug, Formatter},
};

mod display;

impl ChemicalTerm {
    pub fn atom<S>(atom: S) -> Self
    where
        S: Into<String>,
    {
        ChemicalTerm { kind: ChemicalKind::Atomic(atom.into()), compound: vec![], count: 1.0, electronic: 0.0 }
    }
    pub fn compound(compound: Vec<ChemicalTerm>) -> Self {
        ChemicalTerm { kind: ChemicalKind::Compound, compound, count: 1.0, electronic: 0.0 }
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
    pub fn get_kind(&self) -> &ChemicalKind {
        &self.kind
    }
    pub fn set_kind(&mut self, kind: ChemicalKind) {
        self.kind = kind;
    }
    pub fn with_kind(mut self, kind: ChemicalKind) -> Self {
        self.kind = kind;
        self
    }
    pub fn get_atom(&self) -> &str {
        match &self.kind {
            ChemicalKind::Atomic(atom) => atom,
            ChemicalKind::Paired(_, _) => "",
            ChemicalKind::Compound => "",
        }
    }
    pub fn get_compound(&self) -> &[ChemicalTerm] {
        if self.is_atom() {
            return &[];
        }
        &self.compound
    }
    pub fn get_number(&self) -> f64 {
        self.count
    }
    pub fn set_number(&mut self, count: f64) {
        self.count = count;
    }
    pub fn with_number(mut self, count: f64) -> Self {
        self.count = count;
        self
    }
    pub fn get_electronic(&self) -> f64 {
        self.electronic
    }
    pub fn set_electronic(&mut self, e: f64) {
        self.electronic = e;
    }
    pub fn with_electronic(mut self, e: f64) -> Self {
        self.electronic = e;
        self
    }
}

impl ChemicalBalancer {
    pub fn get_elements(&self) -> &BTreeSet<String> {
        &self.elements
    }
    pub fn record_elements(&mut self) {
        for i in self.lhs.iter().chain(self.rhs.iter()) {
            i.record_elements(&mut self.elements);
        }
    }
    pub fn count_elements(&self, compound: &ChemicalTerm) -> Vec<f64> {
        let mut out = vec![0.0; self.elements.len()];
        compound.count_elements(&self.elements, &mut out, 1.0);
        out
    }
}

impl ChemicalTerm {
    pub fn record_elements(&self, all: &mut BTreeSet<String>) {
        match &self.kind {
            ChemicalKind::Atomic(s) => {
                all.insert(s.clone());
            }
            ChemicalKind::Compound | ChemicalKind::Paired(_, _) => {
                if !self.electronic.is_zero() {
                    all.insert("e".to_string());
                }
                for term in &self.compound {
                    term.record_elements(all);
                }
            }
        }
    }
    pub fn count_elements(&self, all: &BTreeSet<String>, out: &mut Vec<f64>, multiplier: f64) {
        let count = self.count * multiplier;
        match all.iter().position(|v| v == "e") {
            Some(s) => {
                out[s] += self.electronic * multiplier;
            }
            None => {}
        }
        match &self.kind {
            ChemicalKind::Atomic(s) => {
                let i = all.iter().position(|v| v == s).unwrap();
                out[i] += count;
            }
            ChemicalKind::Compound | ChemicalKind::Paired(_, _) => {
                for term in &self.compound {
                    term.count_elements(all, out, count);
                }
            }
        }
    }
}
