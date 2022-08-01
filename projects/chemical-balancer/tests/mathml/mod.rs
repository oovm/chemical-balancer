use crate::display_mathml;
use chemical_balancer::{ChemicalBalancer, ChemicalTerm};
use mathml_core::{helpers::assert_no_ws, MathML};
use std::str::FromStr;

#[test]
pub fn test_ph2() {
    let ph = ChemicalTerm::from_str("Ph").unwrap();
    assert_eq!(format!("{}", MathML::from(ph)), r#"<mi mathvariant="normal">Ph</mi>"#);
    let ph2 = ChemicalTerm::from_str("Ph2").unwrap();
    assert_eq!(format!("{}", MathML::from(ph2)), r#"<msub><mi mathvariant="normal">Ph</mi><mn>2</mn></msub>"#);
    let c6h5 = ChemicalTerm::from_str("C6H5COOH").unwrap();
    assert_no_ws(&display_mathml(c6h5), include_str!("C6H5COOH.xml"))
}

#[test]
pub fn test_benzoic_acid() {
    let input = ChemicalBalancer::from_str("C6H5COOH + O2 = CO2 + H2O").unwrap();
    println!("{}", input.solve_by_mathematica());
    let solved = cast_isize_to_f64(input.solve_integers());
    let mathml = input.render_mathml(&solved);
    assert_no_ws(&display_mathml(mathml), include_str!("equation1.xml"))
}

#[test]
pub fn test_benzoic_acid2() {
    let input = ChemicalBalancer::from_str("C + CO + CO2 = O2").unwrap();
    println!("{}", input.solve_by_mathematica());
    let solved = cast_isize_to_f64(input.solve_integers());
    let mathml = input.render_mathml(&solved);
    assert_no_ws(&display_mathml(mathml), include_str!("equation2.xml"))
}

fn cast_isize_to_f64(v: Vec<Vec<isize>>) -> Vec<Vec<f64>> {
    v.iter().map(|v| v.iter().map(|&x| x as f64).collect()).collect()
}
