use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "dev")] {
        pub const MEDIA_URL:&'static str = "http://localhost:4102";
    } else {
        pub const MEDIA_URL:&'static str = "https://storage.googleapis.com/bytes-ji-kids-eu";
    } 
}


pub const HOST_DIRECTORY:Option<&'static str> = None;