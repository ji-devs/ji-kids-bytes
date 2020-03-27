use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator::{Dom, html, clone};
use dominator_helpers::{html_at_slot};
use std::rc::Rc;
use crate::events::*;
use crate::enums::*;
use manifest::*;

pub struct TopicLanding {
    section: Mutable<Section>,
    manifest: Rc<TopicManifest>
}

impl TopicLanding {
    pub fn new(section: Section, manifest: Rc<TopicManifest>) -> Self {
        Self {
            section: Mutable::new(section),
            manifest 
        }
    }

    pub fn render(topic_landing: Rc<Self>) -> Dom {
        html!("topic-landing", {
            .attribute_signal("section", Self::section_name_signal(topic_landing.clone()))
            .attribute("title", &topic_landing.manifest.meta.title)
            .attribute("id", &topic_landing.manifest.meta.id)
            .event(clone!(topic_landing => move |event: SelectSectionEvent| {
                let section:Section = event.data().section.into();
                topic_landing.section.set(section);
            }))
            .child_signal(Self::section_dom_signal(topic_landing.clone()))
        })
    }

    fn section_name_signal(self: Rc<Self>) -> impl Signal<Item = String> {
        self.section.signal().map(|section| section.into())
    }

    fn section_dom_signal(self: Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let self_clone = self.clone();
        self.section.signal().map(move |section| {
            let topic_id = &self.manifest.meta.id;

            match section {
                Section::Watch => {
                    let videos = serde_wasm_bindgen::to_value(&self_clone.manifest.videos).unwrap();

                    Some(html_at_slot!("section-media", "section", {
                        .property("medias", &videos) 
                        .attribute("topic_id", topic_id) 
                    }))
                },
                Section::Games=> {
                    let games = serde_wasm_bindgen::to_value(&self_clone.manifest.games).unwrap();

                    Some(html_at_slot!("section-media", "section", {
                        .property("medias", &games) 
                        .attribute("topic_id", topic_id) 
                    }))
                },
                Section::Discover=> {
                    let discovers = serde_wasm_bindgen::to_value(&self_clone.manifest.discovers).unwrap();

                    Some(html_at_slot!("section-links", "section", {
                        .attribute("section", "discover") 
                        .property("links", &discovers) 
                        .attribute("topic_id", topic_id) 
                    }))
                },
                Section::Create=> {
                    let create = serde_wasm_bindgen::to_value(&self_clone.manifest.create).unwrap();

                    Some(html_at_slot!("section-create", "section", {
                        .property("create", &create) 
                        .attribute("topic_id", topic_id) 
                    }))
                },
                Section::Craft=> {
                    let crafts = serde_wasm_bindgen::to_value(&self_clone.manifest.crafts).unwrap();

                    Some(html_at_slot!("section-links", "section", {
                        .attribute("section", "crafts") 
                        .property("links", &crafts) 
                        .attribute("topic_id", topic_id) 
                    }))
                },
                Section::Help=> {
                    Some(html_at_slot!("section-help", "section", {
                        .attribute("topic_id", topic_id) 
                    }))
                },
                _ => None
            }
        })
        //
    }
}
