use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};
use crate::{ChemicalBalancer, Compound, CompoundGroup};


impl ChemicalBalancer {
    pub fn count_elements(&self, compound: &Compound) -> Vec<f64> {
        compound.count_elements(&self.elements)
    }
    pub fn update_elements(&mut self) {
        for i in &self.lhs {
            i.record_elements(&mut self.elements);
        }
        for i in &self.rhs {
            i.record_elements(&mut self.elements);
        }
    }
}


// co2

impl Debug for Compound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Compound::Atom { atom, count, .. } => {
                match count.partial_cmp(&1.0) {
                    Some(Ordering::Equal) =>
                        write!(f, "{}", atom)

                    ,
                    _ =>
                        write!(f, "{}{}", atom, count)
                }
            }
            Compound::Compound { compound, count, .. } => {
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

impl Compound {
    pub fn record_elements(&self, all: &mut BTreeSet<String>) {
        match self {
            Compound::Atom { atom, .. } => {
                all.insert(atom.clone());
            }
            Compound::Compound { compound, .. } => {
                for c in compound {
                    c.record_elements(all);
                }
            }
        }
    }
    pub fn count_elements(&self, all: &BTreeSet<String>) -> Vec<f64> {
        match self {
            Compound::Atom { atom, count, .. } => {
                let mut out = vec![0.0; all.len()];
                if let Some(i) = all.iter().position(|x| x == atom) {
                    out[i] = *count;
                    out
                } else {
                    out
                }
            }
            Compound::Compound { compound, count, .. } => {
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

impl Compound {
    pub fn atom(atom: String, count: f64) -> Self {
        Compound::Atom {
            atom,
            count,
            electronic: 0.0,
        }
    }

    pub fn compound(compound: Vec<Compound>, count: f64) -> Self {
        Compound::Compound {
            group: CompoundGroup::None,
            compound,
            count,
            electronic: 0.0,
        }
    }
    pub fn parentheses(compound: Vec<Compound>, count: f64) -> Self {
        Compound::Compound {
            group: CompoundGroup::Parentheses,
            compound,
            count,
            electronic: 0.0,
        }
    }
    pub fn with_electronic(mut self, e: f64) -> Self {
        match &mut self {
            Compound::Atom { electronic, .. } => {
                *electronic = e;
            }
            Compound::Compound { electronic, .. } => {
                *electronic = e;
            }
        }
        self
    }
}

