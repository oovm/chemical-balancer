use crate::{ChemicalBalancer, ChemicalTerm};

mod display;
mod solver;
use num::{Integer, One};
use rationalize::float2ratio;

impl ChemicalBalancer {
    pub fn compounds(&self) -> impl Iterator<Item = &ChemicalTerm> {
        self.equation.iter()
    }
}
