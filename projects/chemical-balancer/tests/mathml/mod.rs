use crate::assert_by_ws;
use chemical_balancer::{
    helpers::{cast_isize_to_f64, display_mathml},
    ChemicalBalancer, ChemicalTerm,
};
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

#[test]
pub fn test_benzoic_acid3() {
    // K4Fe(CN)6 + KMnO4 + H2SO4 = KHSO4 + Fe2(SO4)3 + MnSO4 + HNO3 + CO2 + H2O
    const EQ: &'static str = "K4Fe(CN)6+KMnO4+H2SO4=CO2+KNO3+H2O+K2SO4+MnSO4+Fe2(SO4)3";
    let input = ChemicalBalancer::from_str(EQ).unwrap();
    println!("{}", input.solve_by_mathematica());
    let solved = cast_isize_to_f64(input.solve_integers());
    let mathml = input.render_mathml(&solved);
    assert_no_ws(&display_mathml(mathml), include_str!("equation3.xml"))
}
