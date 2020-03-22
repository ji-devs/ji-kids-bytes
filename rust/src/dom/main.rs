use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator::{Dom, html, clone};
use dominator_helpers::{html_at_slot};
use std::rc::Rc;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use crate::events::*;
use crate::loader::*;
use crate::enums::*;
use crate::schema::*;
use super::*;

pub struct Main {
    section: Mutable<Section>,
    manifest: Manifest 
}

impl Main {
    pub fn new(section: Section, manifest: Manifest) -> Self {
        Self {
            section: Mutable::new(section),
            manifest 
        }
    }

    pub fn render(main: Rc<Self>) -> Dom {
        html!("app-main", {
            .attribute_signal("section", main.clone().section_signal())
            .attribute("meta-title", &main.manifest.meta.title)
            .attribute("meta-color", &main.manifest.meta.title)
            .event(clone!(main=> move |event: SelectSectionEvent| {
                let section:Section = event.data().section.into();
                main.section.set(section);
            }))
        })
    }

    fn section_signal(self: Rc<Self>) -> impl Signal<Item = String> {
        self.section.signal().map(|section| section.into())
    }
}
