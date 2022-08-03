use crate::{ChemicalBalancer, ChemicalTerm};
use latexify::Latexify;
use mathml_core::{helpers::cases, MathML, MathNumber, MathOperator, MathRow};
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
            let mut lines = vec![];
            for coefficients in solved {
                let line = self.render_mathml_row(coefficients);
                lines.push(line);
                lines.push(MathML::NewLine)
            }
            cases(lines)
        }
        else {
            let coefficients = solved[0].as_slice();
            self.render_mathml_row(coefficients)
        }
    }

    fn render_mathml_row(&self, coefficients: &[f64]) -> MathML {
        let mut lhs = Vec::new();
        let mut rhs = Vec::new();
        for (index, &coefficient) in coefficients.iter().enumerate() {
            let item = self.get_term(index).expect("item index out of range");
            if coefficient > 0.0 {
                rhs.push((coefficient, item));
            }
            else if coefficient < 0.0 {
                lhs.push((-coefficient, item));
            }
            else {
                // zero, drop
            }
        }
        let mut eqation: Vec<MathML> = vec![];
        push_pair(&mut eqation, lhs);
        eqation.push(MathOperator::new("=").into());
        push_pair(&mut eqation, rhs);
        MathRow::new(eqation).into()
    }
}

fn push_pair(eqation: &mut Vec<MathML>, terms: Vec<(f64, ChemicalTerm)>) {
    for (index, (coefficient, item)) in terms.into_iter().enumerate() {
        if index != 0 {
            eqation.push(MathOperator::new("+").into())
        }
        if coefficient != 1.0 {
            eqation.push(MathNumber::new(coefficient).into());
        }
        eqation.push(item.into())
    }
}
