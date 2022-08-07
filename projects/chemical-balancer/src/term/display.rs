use super::*;
use latexify::Latexify;
use mathml_core::{MathFenced, MathIdentifier, MathML, MathMultiScript, MathRow};
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

impl From<ChemicalTerm> for MathML {
    fn from(value: ChemicalTerm) -> Self {
        let terms = value.compound.into_iter().map(|term| term.into());
        match &value.kind {
            ChemicalKind::Atomic(atom) => {
                let base = MathIdentifier::normal(atom);
                if value.count == 1.0 {
                    base.into()
                }
                else {
                    MathMultiScript::sub_script(base.into(), value.count.into()).into()
                }
            }
            ChemicalKind::Compound => MathRow::new(terms).into(),
            ChemicalKind::Paired(lhs, rhs) => MathFenced::new(terms, *lhs, *rhs).into(),
            ChemicalKind::Attached(_) => {
                todo!()
            }
        }
    }
}

impl Latexify for ChemicalTerm {
    fn fmt<W: Write>(&self, f: &mut W) -> std::fmt::Result {
        f.write_str("\\ce{")?;
        f.write_str(&self.to_string())?;
        f.write_str("}")
    }
}
