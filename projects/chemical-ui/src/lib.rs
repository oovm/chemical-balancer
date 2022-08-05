#![allow(non_snake_case)]

use dioxus::html::textarea;
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use crate::hooks::use_chemical_balancer;
use dioxus::prelude::*;
use dioxus_use_storage::use_local_storage;

mod hooks;

// create a component that renders a div with the text "Hello, world!"
pub fn App(cx: Scope) -> Element {
    let chem = use_chemical_balancer(&cx);
    cx.render(rsx! {
        div {
            h2 { "Chemical Balancer" }
            input {
                oninput: move |evt| chem.on_input(evt),
            }
            h2 { "Chemical 2"}
            textarea {

            }
            h2 {
                "Balanced 3"
            }
            textarea {

            }
        }
    })
}
