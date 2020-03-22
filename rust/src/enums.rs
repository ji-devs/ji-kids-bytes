#[derive(Clone, Copy, PartialEq)]
pub enum Section {
    Watch,
    Games,
    Create,
    Look,
    HandsOn
}

impl From<Section> for String {
    fn from(section:Section) -> String {
        match section {
            Section::Watch => "watch".to_string(),
            Section::Games => "games".to_string(),
            Section::Create => "create".to_string(),
            Section::Look => "look".to_string(),
            Section::HandsOn => "hands-on".to_string(),
        }
    }
}

impl <T: AsRef<str>> From<T> for Section {
    fn from(section:T) -> Section {
        match section.as_ref() {
            "watch" => Section::Watch,
            "games" => Section::Games,
            "create" => Section::Create,
            "look" => Section::Look,
            "hands-on" => Section::HandsOn,
            _ => panic!("unsupported section string!")
        }
    }
}