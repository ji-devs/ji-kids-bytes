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
use super::main::Main;
use crate::enums::*;
use crate::manifest::*;
use crate::config;

pub struct Root {
    pub page: Mutable<RootPage>,
}

#[derive(Clone)]
pub enum RootPage {
    Loading,
    Main(Manifest),
    Home,
    NotFound,
}

impl Root {
    pub fn new() -> Self {
        Self { 
            page: Mutable::new(RootPage::Loading),
        }
    }
}


impl Root {
    pub fn render(root: Rc<Self>) -> Dom {
        html!("main", {
            .child_signal(Self::page_signal(root).map(|page| {
                Some(page)
            }))
        })
    }

    fn page_signal(root: Rc<Self>) -> impl Signal<Item = Dom> {
        root.page.signal_cloned().map(|page| match page {
            RootPage::Loading => html!("app-loading"),
            RootPage::Home => html!("app-home"),
            RootPage::Main(manifest) => {
                let main = Rc::new(Main::new(config::DEFAULT_SECTION, manifest));
                Main::render(main)
            },
            _ => html!("not-found")
        })
    }
}