use web_sys::window;
use wasm_bindgen::prelude::*;
use crate::config;

pub fn get_topic() -> String {

    fn get_pathname() -> Result<String, JsValue> {
        let pathname = window().unwrap().location().pathname()?;

        let pathname = get_root(pathname.as_str());

        Ok(pathname.to_string())
    }

    let pathname = get_pathname().unwrap_or("".to_string());

    pathname
}

//simple stripping of host dir like if deploying to example.com/foo
fn get_root(input: &str) -> &str {
    let stripped = match config::HOST_DIRECTORY {
        None => input,
        Some(host_dir) => {
            input
                .find(host_dir)
                .map(|len| input.split_at(len + host_dir.len() - 1).1)
                .or(Some(input))
                .unwrap()
        }
    };

    stripped.trim_matches('/')
}