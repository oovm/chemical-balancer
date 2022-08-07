#![allow(unused, dead_code)]
use chemical_balancer::ChemicalBalancer;
use mathml_core::{MathML, MathRoot};
use std::str::FromStr;

mod mathml;
mod simple;

#[test]
fn ready() {
    println!("it works!")
}

pub fn assert_by_ws(math: &MathML, target: &str) {
    assert_eq!(math.to_string(), target);
}
