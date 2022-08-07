#![allow(non_snake_case)]

use dioxus::html::textarea;
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use crate::hooks::use_chemical_balancer;
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
            input {
                placeholder: "Type here",
                value: "{text}",
                oninput: move |e| text.set(e.value.to_owned()),
            }
            h2 { "Balanced Equation" }
            chem.as_mathml()
            h2 {
                "High-Precision Checking"
            }
            chem.as_mathematica()
        }
    })
}
