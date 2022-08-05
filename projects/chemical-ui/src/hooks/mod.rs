use chemical_balancer::ChemicalBalancer;
use dioxus::{
    core::{Event, ScopeState},
    events::FormData,
};
use std::{cell::RefCell, rc::Rc, str::FromStr};

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
    pub fn on_input(&self, data: Event<FormData>) {
        self.update(&data.value);
    }

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
}
