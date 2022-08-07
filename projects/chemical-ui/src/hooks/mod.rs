use chemical_balancer::{
    helpers::{cast_isize_to_f64, display_mathml},
    ChemicalBalancer,
};
use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc, str::FromStr};

#[derive(Clone)]
pub struct UseChemicalBalancer {
    data: Rc<RefCell<BalancerData>>,
}

pub fn use_chemical_balancer(cx: &ScopeState) -> &mut UseChemicalBalancer {
    cx.use_hook(|| {
        let data = Rc::new(RefCell::new(BalancerData {
            string: "".to_string(),
            balancer: ChemicalBalancer::default(),
            error: "".to_string(),
        }));
        UseChemicalBalancer { data }
    })
}

struct BalancerData {
    string: String,
    balancer: ChemicalBalancer,
    error: String,
}

impl UseChemicalBalancer {
    pub fn update(&self, text: &str) {
        let mut data = self.data.borrow_mut();
        data.string = text.to_string();
        match ChemicalBalancer::from_str(&data.string) {
            Ok(o) => {
                data.balancer = o;
                data.error.clear();
            }
            Err(e) => {
                data.error = e.to_string();
            }
        }
    }
    pub fn as_mathml(&self) -> LazyNodes {
        let data = self.data.borrow();
        if !data.error.is_empty() {
            let message = data.error.clone();
            return rsx! {
                div {
                    span {
                        style: "color: red;",
                        message
                    }
                }
            };
        }
        let solved = cast_isize_to_f64(data.balancer.solve_integers());
        let mathml = data.balancer.render_mathml(&solved);
        let raw = display_mathml(mathml).to_string();
        rsx! {
            div {
                dangerous_inner_html: "{raw}"
            }
        }
    }
    pub fn as_mathematica(&self) -> LazyNodes {
        let data = self.data.borrow();
        if !data.error.is_empty() {
            return rsx! { div {} };
        }
        let text = data.balancer.solve_by_mathematica();
        rsx! {
            div {
                pre {
                    text
                }
            }
        }
    }
}
