#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use chemical_ui::Editor;
use dioxus::prelude::*;
fn main() {
    // launch the web app
    dioxus_desktop::launch(Editor);
}
