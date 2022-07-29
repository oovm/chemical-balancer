use super::*;
use chemical_balancer::ChemicalTerm;

#[test]
pub fn test_ph2() {
    let ph2 = ChemicalTerm::from_str("Ph2").unwrap();
    assert_eq!("Ph2", format!("{:?}", ph2));
    let c6h5 = ChemicalTerm::from_str("C6H5COOH").unwrap();
    assert_eq!("Compound(C6, H5, C1, O1, O1, H1, 1.0)", format!("{:?}", c6h5))
}

#[test]
pub fn test_benzoic_acid() {
    let input = ChemicalBalancer::from_str("C6H5COOH + O2 = CO2 + H2O").unwrap();
    // println!("{}", input.solve_by_mathematica());
    let output = vec![vec![-2, -15, 14, 6]];
    assert_eq!(output, input.solve_integers());
}

#[test]
pub fn test_ph_ch3() {
    let input = ChemicalBalancer::from_str("PhCH3 + KMnO4 + H2SO4 = PhCOOH + K2SO4 + MnSO4 + H2O").unwrap();
    let output = vec![vec![-5, -6, -9, 5, 3, 6, 14]];
    assert_eq!(output, input.solve_integers());
}

#[test]
pub fn test_k4_fe_cn6() {
    let input = ChemicalBalancer::from_str("K4Fe(CN)6 + KMnO4 + H2SO4 = KHSO4 + Fe2(SO4)3 + MnSO4 + HNO3 + CO2 + H2O").unwrap();
    // println!("{}", input.latexify());
    // println!("{}", input.solve_by_mathematica());
    let output = vec![vec![-10, -122, -299, 162, 5, 122, 60, 60, 188]];
    assert_eq!(output, input.solve_integers());
}

#[test]
pub fn test_c_co2() {
    let input = ChemicalBalancer::from_str("C + CO + CO2 = O2").unwrap();
    // println!("{}", input.latexify());
    // println!("{}", input.solve_by_mathematica());
    let output = vec![vec![1, -2, 1, 0], vec![2, -2, 0, 1]];
    assert_eq!(output, input.solve_integers());
}
