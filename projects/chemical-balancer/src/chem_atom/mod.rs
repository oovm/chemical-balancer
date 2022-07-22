use crate::{ChemicalBalancer, ChemicalKind, ChemicalTerm};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Formatter},
};
mod display;
mod solver;
use num::{Integer, One};
use rationalize::float2ratio;
