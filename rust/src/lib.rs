//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ts_test")] {
        mod events;
    } else {
        mod dom;
        mod events;
        mod manifest;
        mod loader;
        mod config;
        mod path;
        mod enums;
        mod router;


        use wasm_bindgen::prelude::*;

        #[cfg(feature = "wee_alloc")]
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

        // enable logging and panic hook only during debug builds
        cfg_if! {
            if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook", debug_assertions))] {
                fn setup_logger() {
                    wasm_logger::init(wasm_logger::Config::default());
                    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                    log::info!("rust logging enabled!!!");
                }
            } else {
                fn setup_logger() {
                    log::info!("rust logging disabled!"); //<-- won't be seen
                }
            }
        }

        #[wasm_bindgen(start)]
        pub async fn main_js() -> Result<(), JsValue> {
            use std::rc::Rc;
            use dom::root::{Root, RootPage};

            setup_logger();

            let root = Rc::new(Root::new());
            dominator::append_dom(&dominator::body(), Root::render(Rc::clone(&root)));

            //TODO - move all this into router and return page or something
            let uri_parts = router::get_uri_parts();
            if uri_parts.len() == 0 {
                root.page.set(RootPage::Home);
            } else {
                if uri_parts[0] == "topic" {
                    if uri_parts.len() > 1 {
                        match loader::load_manifest(&uri_parts[1]).await {
                            Ok(manifest) => {
                                root.page.set(RootPage::Main(manifest));
                            },
                            Err(err) => {
                                log::info!("BAD MANIFEST!");
                                root.page.set(RootPage::NotFound);
                            }
                        }
                    } else {
                        root.page.set(RootPage::NotFound);
                    }
                } else {
                    root.page.set(RootPage::NotFound);
                }
            }


            Ok(())

        }
    }
}



