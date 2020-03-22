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
use crate::schema::*;

pub struct Root {
    page: Mutable<RootPage>,
    loader_promise: RefCell<Option<js_sys::Promise>>
}

#[derive(Clone)]
enum RootPage {
    Loading,
    Ready(Manifest),
    Error
}

impl Root {
    pub fn new() -> Self {
        Self { 
            page: Mutable::new(RootPage::Loading),
            loader_promise: RefCell::new(None),
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

    pub fn load(root: Rc<Self>, topic:String) {
        let root_clone = Rc::clone(&root);
        *root_clone.loader_promise.borrow_mut() = Some(future_to_promise(async move {
            match load_manifest(&topic).await {
                Ok(manifest) => {
                    root.page.set(RootPage::Ready(manifest));
                },
                Err(err) => {
                    root.page.set(RootPage::Error)
                }
            }

            root.loader_promise.borrow_mut().take();
            Ok(JsValue::null())
        }));
    }

    fn page_signal(root: Rc<Self>) -> impl Signal<Item = Dom> {
        root.page.signal_cloned().map(|page| match page {
            RootPage::Loading => html!("app-loading"),
            RootPage::Ready(manifest) => {
                let main = Rc::new(Main::new(Section::Watch, manifest));
                Main::render(main)
            },
            _ => html!("app-error")
        })
    }
}