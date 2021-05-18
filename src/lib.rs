use core::str;
use psl::{List, Psl};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn suffix(name: &str) -> Option<String> {
    let suffix = List.suffix(name.as_bytes())?;
    let suffix = str::from_utf8(suffix.as_bytes()).ok()?;
    Some(suffix.to_owned())
}

#[wasm_bindgen]
pub fn domain(name: &str) -> Option<String> {
    let domain = List.domain(name.as_bytes())?;
    let domain = str::from_utf8(domain.as_bytes()).ok()?;
    Some(domain.to_owned())
}
