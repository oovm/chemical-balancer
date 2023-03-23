use chemical_balancer::ChemicalBalancer;
use std::str::FromStr;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let co2 = ChemicalBalancer::from_str("C6H5COOH + O2 = CO2 + H2O").unwrap();
    // println!("{:#?}", co2.count_elements());
    println!("{:#?}", co2.solve());
    println!("{:#?}", co2.solve_integers())
}
