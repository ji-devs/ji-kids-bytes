use cfg_if::cfg_if;
use crate::enums::Section;

cfg_if! {
    if #[cfg(feature = "dev")] {
        pub const MEDIA_URL:&'static str = "http://localhost:4102";
        pub const DEFAULT_SECTION:Section = Section::Discover;
    } else {
        pub const MEDIA_URL:&'static str = "https://storage.googleapis.com/bytes-ji-kids-eu";
        pub const DEFAULT_SECTION:Section = Section::Watch;
    } 
}


pub const HOST_DIRECTORY:Option<&'static str> = None;
