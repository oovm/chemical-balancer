use chemical_balancer::ChemicalBalancer;
use std::str::FromStr;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let co2 = ChemicalBalancer::from_str("KMnO4 + HCl = KCl + MnCl2 + H2O + Cl2").unwrap();
    println!("{:#?}", co2.count_elements());
    let sol = co2.solve_integers();
    println!("{:#?}", sol)
}
