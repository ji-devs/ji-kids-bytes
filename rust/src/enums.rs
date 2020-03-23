use std::rc::Rc;
use crate::manifest::{AppManifest, TopicManifest};

#[derive(Clone)]
pub enum RootPage {
    Loading,
    Topic(Rc<TopicManifest>),
    Home(Rc<AppManifest>),
    NotFound,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Section {
    Watch,
    Games,
    Discover,
    Create,
    Craft 
}

impl From<Section> for String {
    fn from(section:Section) -> String {
        match section {
            Section::Watch => "watch".to_string(),
            Section::Games => "games".to_string(),
            Section::Discover=> "discover".to_string(),
            Section::Create=> "create".to_string(),
            Section::Craft=> "craft".to_string(),
        }
    }
}

impl <T: AsRef<str>> From<T> for Section {
    fn from(section:T) -> Section {
        match section.as_ref() {
            "watch" => Section::Watch,
            "games" => Section::Games,
            "discover" => Section::Discover,
            "create" => Section::Create,
            "craft" => Section::Craft,
            _ => panic!("unsupported section string!")
        }
    }
}
