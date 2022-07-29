use chemical_balancer::ChemicalTerm;
use mathml_core::{helpers::assert_no_ws, MathML, MathRoot};
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

pub fn display_mathml<M>(math: M) -> MathML
where
    M: Into<MathML>,
{
    MathML::from(MathRoot::new(vec![math.into()]).with_namespace().with_display_style(true))
}
