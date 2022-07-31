use crate::ChemicalBalancer;
use latexify::Latexify;
use mathml_core::{MathML, MathNumber, MathOperator, MathRow};
use std::fmt::Write;

impl Latexify for ChemicalBalancer {
    fn fmt<W: Write>(&self, f: &mut W) -> std::fmt::Result {
        for (index, term) in self.equation.iter().enumerate() {
            if index != 0 {
                f.write_str(" + ")?;
            }
            Latexify::fmt(term, f)?;
        }
        Ok(())
    }
}

impl ChemicalBalancer {
    pub fn render_mathml(&self, solved: &[Vec<f64>]) -> MathML {
        if solved.len() > 1 {
            todo!()
        }
        else {
            let coefficients = solved[0].as_slice();
            let mut lhs = Vec::new();
            let mut rhs = Vec::new();
            for (index, coefficient) in coefficients.iter().enumerate() {
                let item = self.get_term(index).expect("item index out of range");
                if coefficient.is_sign_positive() {
                    rhs.push((coefficient, item));
                }
                else if coefficient.is_sign_negative() {
                    lhs.push((-coefficient, item));
                }
                else {
                    // zero, drop
                }
            }
            let mut eqation: Vec<MathML> = vec![];
            for (index, (coefficient, item)) in lhs.into_iter().enumerate() {
                if index != 0 {
                    eqation.push(MathOperator::new("+").into())
                }
                eqation.push(MathNumber::new(coefficient).into());
                eqation.push(item.into())
            }
            eqation.push(MathOperator::new("=").into());
            for (index, (coefficient, item)) in rhs.into_iter().enumerate() {
                if index != 0 {
                    eqation.push(MathOperator::new("+").into())
                }
                eqation.push(MathNumber::new(coefficient).into());
                eqation.push(item.into())
            }
            MathRow::new(eqation).into()
        }
    }
}
