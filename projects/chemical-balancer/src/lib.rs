mod errors;
mod chem_atom;
mod parser;

use std::collections::BTreeSet;
pub use errors::{Error, Result};


pub struct ChemicalBalancer {
    elements: BTreeSet<String>,
    lhs: Vec<Compound>,
    rhs: Vec<Compound>,
}

pub enum Compound {
    Atom {
        atom: String,
        count: f64,
    },
    Compound {
        group: CompoundGroup,
        compound: Vec<Compound>,
        count: f64,
    },
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundGroup {
    #[default]
    None,
    Parentheses,
}
