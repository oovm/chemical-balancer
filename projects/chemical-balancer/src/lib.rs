mod chem_atom;
mod parser;
mod term;

use std::collections::BTreeSet;

#[derive(Debug)]
pub struct ChemicalBalancer {
    elements: BTreeSet<String>,
    lhs: Vec<ChemicalTerm>,
    rhs: Vec<ChemicalTerm>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChemicalTerm {
    kind: ChemicalKind,
    compound: Vec<ChemicalTerm>,
    count: f64,
    electronic: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChemicalKind {
    Atomic(String),
    Paired(char, char),
}
