use dominator::{Dom, html};
use std::rc::Rc;
use manifest::*;

pub struct Home {
    manifest: Rc<AppManifest>
}

impl Home {
    pub fn new(manifest: Rc<AppManifest>) -> Self {
        Self {
            manifest 
        }
    }

    pub fn render(self: Rc<Self>) -> Dom {
        let topics = serde_wasm_bindgen::to_value(&self.manifest.topics).unwrap();
        
        html!("home-landing", {
            .property("topics", &topics) 
        })
    }
}
