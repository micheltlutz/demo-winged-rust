//! An about.me page built with [`winged_rust`], rendered from two places.
//!
//! - [`page::document`] builds the page. It is the only markup in the crate.
//! - The `build-site` binary renders it natively and writes `dist/`.
//! - [`render_profile`] renders the very same tree inside the browser, from WebAssembly.
//!
//! Both paths call the same function with the same inputs, so both produce the same bytes
//! — which is what the badge on the deployed page checks, out loud, on every visit.

pub mod page;
pub mod profile;

/// Renders the profile page and hands the markup back to JavaScript.
///
/// Exported as `renderProfile` — the browser build calls it, compares the result with the
/// copy the native build wrote to `parity/native.txt`, and reports whether they match.
#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = renderProfile)]
pub fn render_profile() -> String {
    page::document().render()
}
