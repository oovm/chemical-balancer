use super::*;
use latexify::Latexify;
use num::One;
use std::fmt::{Display, Write};

impl Debug for ChemicalTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ChemicalKind::Atomic(atom) => {
                write!(f, "{atom}{}", self.count)
            }
            ChemicalKind::Compound | ChemicalKind::Paired(_, _) | ChemicalKind::Attached(_) => {
                let mut v = &mut f.debug_tuple("Compound");
                for item in &self.compound {
                    v = v.field(item);
                }
                v = v.field(&self.count);
                v.finish()
            }
        }
    }
}

impl Display for ChemicalTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ChemicalKind::Compound => {
                for item in &self.compound {
                    Display::fmt(item, f)?;
                }
            }
            ChemicalKind::Paired(l, r) => {
                f.write_char(*l)?;
                for item in &self.compound {
                    Display::fmt(item, f)?;
                }

                f.write_char(*r)?;
            }
            ChemicalKind::Atomic(s) => {
                f.write_str(s)?;
            }
            ChemicalKind::Attached(s) => {
                f.write_char(*s)?;
                for item in &self.compound {
                    Display::fmt(item, f)?;
                }
            }
        }
        if !self.count.is_one() {
            write!(f, "{}", self.count)?;
        }
        if !self.electronic.is_zero() {
            write!(f, "^{{{}}}", self.electronic)?;
        }

        Ok(())
    }
}

impl Latexify for ChemicalTerm {
    type Context = ();

    fn latexify(&self) -> String {
        format!("\\ce{{{}}}", self)
    }
}
