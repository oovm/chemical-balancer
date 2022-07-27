mod chem_atom;
mod parser;
mod term;

use std::collections::BTreeSet;

#[derive(Debug)]
pub struct ChemicalBalancer {
    elements: BTreeSet<String>,
    equation: Vec<ChemicalTerm>,
}

#[derive(Clone, PartialEq)]
pub struct ChemicalTerm {
    kind: ChemicalKind,
    compound: Vec<ChemicalTerm>,
    count: f64,
    electronic: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChemicalKind {
    Atomic(String),
    Compound,
    Paired(char, char),
    Attached(char),
}
