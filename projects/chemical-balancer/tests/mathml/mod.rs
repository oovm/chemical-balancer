use chemical_balancer::ChemicalTerm;
use mathml_core::MathML;
use std::str::FromStr;

#[test]
pub fn test_ph2() {
    let ph = ChemicalTerm::from_str("Ph").unwrap();
    assert_eq!(format!("{}", MathML::from(ph)), r#"<mrow><msub><mi mathvariant="normal">Ph</mi><mn>1</mn></msub></mrow>"#);
    let ph2 = ChemicalTerm::from_str("Ph2").unwrap();
    assert_eq!(format!("{}", MathML::from(ph2)), r#"<mrow><msub><mi mathvariant="normal">Ph</mi><mn>2</mn></msub></mrow>"#);
    // let c6h5 = ChemicalTerm::from_str("C6H5COOH").unwrap();
    // assert_eq!("Compound(C6, H5, C1, O1, O1, H1, 1.0)", format!("{:?}", c6h5))
}
