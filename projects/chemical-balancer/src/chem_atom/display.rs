use crate::{ChemicalBalancer, ChemicalKind, ChemicalTerm};
use latexify::Latexify;
use mathml_core::{MathIdentifier, MathML, MathMultiScript, MathRow};
use std::fmt::Write;

impl Latexify for ChemicalBalancer {
    fn fmt<W: Write>(&self, f: &mut W) -> std::fmt::Result {
        // let mut out = String::new();
        // for (index, term) in self.equation.iter().enumerate() {
        //     if index != 0 {
        //         out.push_str(" + ");
        //     }
        //     out.push_str(&term.latexify());
        // }
        // out
        todo!()
    }
}

impl From<ChemicalTerm> for MathML {
    fn from(value: ChemicalTerm) -> Self {
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
            ChemicalKind::Compound => {
                let terms = value.compound.into_iter().map(|term| term.into());
                let mrow = MathRow::new(terms);
                mrow.into()
            }
            ChemicalKind::Paired(_, _) => {
                todo!()
            }
            ChemicalKind::Attached(_) => {
                todo!()
            }
        }
    }
}
