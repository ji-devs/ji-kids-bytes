//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ts_test")] {
        mod events;
    } else {
        mod dom;
        mod events;
        mod schema;
        mod loader;
        mod config;
        mod path;
        mod enums;
        mod router;

        use wasm_bindgen::prelude::*;
        use std::rc::Rc;
        use dom::root::Root;

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

        #[wasm_bindgen]
        pub fn init_app() {
            setup_logger();

            let root = Rc::new(Root::new());
            Root::load(Rc::clone(&root), router::get_topic());
            dominator::append_dom(&dominator::body(), Root::render(root));

        }
    }
}



