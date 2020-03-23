use dominator::{Dom, html};
use std::rc::Rc;
use crate::manifest::*;

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
        let topics = serde_json::to_string(&self.manifest.topics).unwrap();
        
        html!("home-landing", {
            .attribute("topics", &topics) 
        })
    }
}
