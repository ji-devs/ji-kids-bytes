use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator::{Dom, html};
use std::rc::Rc;
use crate::enums::*;
use crate::config;
use super::pages::*;

pub struct Root {
    pub page: Mutable<RootPage>,
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
            RootPage::Loading => loading::Loading::render(),
            RootPage::Home(manifest) => {
                Rc::new(home::Home::new(manifest.clone())).render()
            },
            RootPage::Topic(manifest) => {
                let topic_landing = Rc::new(topic::TopicLanding::new(config::DEFAULT_SECTION, manifest.clone()));
                topic::TopicLanding::render(topic_landing)
            },
            _ => not_found::NotFound::render()
        })
    }
}