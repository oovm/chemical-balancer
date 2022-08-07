use chemical_balancer::{helpers::cast_isize_to_f64, ChemicalBalancer};
use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc, str::FromStr, sync::Arc};

#[derive(Clone)]
pub struct UseChemicalBalancer {
    data: Rc<RefCell<BalancerData>>,
    notify: Arc<dyn Fn() + Send + Sync + 'static>,
}

pub fn use_chemical_balancer(cx: &ScopeState) -> &mut UseChemicalBalancer {
    cx.use_hook(|| {
        let data = Rc::new(RefCell::new(BalancerData {
            string: "".to_string(),
            balancer: ChemicalBalancer::default(),
            error: "".to_string(),
        }));
        UseChemicalBalancer { data, notify: cx.schedule_update() }
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
            }
            Err(e) => {
                data.error = e.to_string();
            }
        }
    }
    pub fn as_mathml(&self) -> LazyNodes {
        let data = self.data.borrow();
        let solved = cast_isize_to_f64(data.balancer.solve_integers());
        let mathml = data.balancer.render_mathml(&solved).to_string();
        rsx! {
            div {
                dangerous_inner_html: "{mathml}"
            }
        }
    }

    pub fn as_mathematica(&self) -> LazyNodes {
        let data = self.data.borrow();
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
