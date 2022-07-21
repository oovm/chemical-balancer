mod atom;
mod chem_atom;
mod errors;
mod parser;

pub use errors::{Error, Result};
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct ChemicalBalancer {
    elements: BTreeSet<String>,
    lhs: Vec<ChemicalTerm>,
    rhs: Vec<ChemicalTerm>,
}

pub struct Atom {
    atom: String,
    count: f64,
    electronic: f64,
}

pub enum ChemicalTerm {
    Atom(Atom),
    Compound { group: CompoundGroup, compound: Vec<ChemicalTerm>, count: f64, electronic: f64 },
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundGroup {
    #[default]
    None,
    Parentheses,
}
