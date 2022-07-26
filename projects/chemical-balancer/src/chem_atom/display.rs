use crate::ChemicalBalancer;
use latexify::Latexify;

impl Latexify for ChemicalBalancer {
    type Context = ();

    fn latexify(&self) -> String {
        let mut out = String::new();
        for (index, term) in self.equation.iter().enumerate() {
            if index != 0 {
                out.push_str(" + ");
            }
            out.push_str(&term.latexify());
        }
        out
    }
}
