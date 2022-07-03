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
        compound: Vec<Compound>,
        count: f64,
    },
}