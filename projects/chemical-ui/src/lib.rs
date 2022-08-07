#![allow(non_snake_case)]

pub use crate::hooks::{use_chemical_balancer, UseChemicalBalancer};
use dioxus::prelude::*;
mod hooks;

// create a component that renders a div with the text "Hello, world!"
pub fn Editor(cx: Scope) -> Element {
    const PLACE_HOLDER: &str = "C6H5COOH + O2 = CO2 + H2O";
    let text = use_state(&cx, || PLACE_HOLDER.to_string());
    let chem = use_chemical_balancer(&cx);
    chem.update(&text);
    cx.render(rsx! {
        div {
            h2 { "Chemical Balancer" }
            div {
                // align center
                style: "display: flex; justify-content: center;",
                textarea {
                    style: "width: 90%;",
                    rows: 3,
                    placeholder: "{PLACE_HOLDER}",
                    value: "{text}",
                    oninput: move |e| text.set(e.value.to_owned()),
                }
            }
            h3 { "Balanced Equation" }
            chem.as_mathml()
            h3 {
                "High-Precision Checking"
            }
            chem.as_mathematica()
        }
    })
}
