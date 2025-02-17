pub mod dom;
pub mod fetch;
#[cfg(any(feature = "serde-json", feature = "serde-wasm-bindgen"))]
mod json;
pub mod service;
pub mod url;
pub mod util;
pub mod web_socket;
pub mod web_storage;

pub use url::{Url, UrlSearch, DUMMY_BASE_URL};
