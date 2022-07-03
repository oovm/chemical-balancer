use std::collections::BTreeSet;




impl ChemicalBalancer {
    pub fn count_elements(&self, compound: &Compound) -> Vec<f64> {
        compound.count_elements(&self.elements)
    }
}




// co2


impl Compound {
    pub fn count_elements(&self, all: &BTreeSet<String>) -> Vec<f64> {
        match self {
            Compound::Atom { atom, count } => {
                let mut out = vec![0.0; all.len()];
                if let Some(i) = all.iter().position(|x| x == atom) {
                    out[i] = *count;
                    out
                } else {
                    out
                }
            }
            Compound::Compound { compound, count } => {
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


