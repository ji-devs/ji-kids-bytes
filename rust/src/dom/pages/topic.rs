use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator::{Dom, html, clone};
use dominator_helpers::{html_at_slot};
use std::rc::Rc;
use crate::events::*;
use crate::enums::*;
use crate::manifest::*;

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
                    let ids:Vec<&str> = self_clone.manifest.videos.iter().map(|vid| vid.id.as_ref()).collect();
                    let ids = serde_json::to_string(&ids).unwrap();

                    Some(html_at_slot!("section-watch", "section", {
                        .attribute("ids_json", &ids) 
                        .attribute("topic_id", topic_id) 
                    }))
                },
                Section::Games=> {
                    let ids:Vec<&str> = self_clone.manifest.games.iter().map(|vid| vid.id.as_ref()).collect();
                    let ids = serde_json::to_string(&ids).unwrap();

                    Some(html_at_slot!("section-games", "section", {
                        .attribute("ids_json", &ids) 
                        .attribute("topic_id", topic_id) 
                    }))
                },
                Section::Discover=> {
                    let discovers = serde_json::to_string(&self_clone.manifest.discovers).unwrap();

                    Some(html_at_slot!("section-discover", "section", {
                        .attribute("discovers_json", &discovers) 
                        .attribute("topic_id", topic_id) 
                    }))
                },
                _ => None
            }
        })
        //
    }
}
