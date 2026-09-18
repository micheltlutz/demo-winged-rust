//! The page — built once, rendered twice.
//!
//! [`document`] is the single source of markup in this demo. The `build-site` binary calls
//! it to write `dist/`, and the WebAssembly module calls it in the browser. Nothing here
//! may depend on the clock, the filesystem or the environment at runtime, or the two
//! renders would drift apart and the parity badge would (correctly) go red.
//!
//! ```
//! let markup = winged_demo::page::document().render();
//! assert!(markup.starts_with("<!DOCTYPE html>"));
//! ```

use winged_rust::prelude::*;
use winged_rust::{Document, seo::SeoBuilder};

use crate::profile::{Link, PROFILE, SITE_URL};

/// The repository this demo advertises in its footer.
pub const LIBRARY_REPO: &str = "https://github.com/micheltlutz/winged-rust";

/// The repository holding this demo itself.
pub const DEMO_REPO: &str = "https://github.com/micheltlutz/demo-winged-rust";

/// The stylesheet, inlined at compile time so the page is a single request.
const CSS: &str = include_str!("style.css");

/// Builds the whole page.
pub fn document() -> Document {
    Document::new(Some("pt-BR"))
        .head_children(
            SeoBuilder::new(
                format!("{} — {}", PROFILE.name, PROFILE.handle),
                PROFILE.headline,
            )
            .image(PROFILE.avatar)
            .url(SITE_URL)
            .site_name(PROFILE.name)
            .author(PROFILE.name)
            .keywords(["winged-rust", "rust", "webassembly", "html", "about.me"])
            .build(),
        )
        .head_children([
            link().attr("rel", "canonical").attr("href", SITE_URL),
            // A one-line emoji favicon: no extra file to deploy, no 404 in the console.
            link().attr("rel", "icon").attr(
                "href",
                "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' \
                 viewBox='0 0 100 100'><text y='.9em' font-size='90'>🪶</text></svg>",
            ),
            // raw_text, not text: CSS is markup-adjacent and must reach the browser
            // unescaped — `>` in a selector would otherwise arrive as `&gt;`.
            style().raw_text(CSS),
        ])
        .body_children([
            div().add_class("shell").children_from([
                profile_header(),
                pull_quote(),
                link_list(),
                parity_panel(),
                page_footer(),
            ]),
            script_src("./boot.js").attr("type", "module"),
        ])
}

/// Avatar, name, handle and bio.
fn profile_header() -> Element {
    header()
        .add_class("profile")
        .set_role("banner")
        .child(
            figure().add_class("avatar").child(
                image(PROFILE.avatar, PROFILE.avatar_alt)
                    .attr("width", "112")
                    .attr("height", "112"),
            ),
        )
        .child(h1().add_class("name").text(PROFILE.name))
        .child(
            p().add_class("meta")
                .child(span().add_class("handle").text(PROFILE.handle))
                .child(span().text(" · "))
                .child(span().text(PROFILE.location)),
        )
        .child(p().add_class("headline").text(PROFILE.headline))
        .child(p().add_class("bio").text(PROFILE.bio))
}

/// The line from the bio that deserves its own space.
fn pull_quote() -> Element {
    blockquote()
        .add_class("quote")
        .child(p().text(PROFILE.quote))
}

/// The list of links, one row per [`Link`] in [`PROFILE`].
fn link_list() -> Element {
    let mut list = ul();
    for entry in PROFILE.links {
        list = list.child(li().child(link_row(entry)));
    }

    nav()
        .add_class("links")
        .set_role("navigation")
        .aria_attr("label", "Links do perfil")
        .child(list)
}

/// One row: initial badge, label, note, arrow.
fn link_row(entry: &Link) -> Element {
    let mut anchor = link_to(entry.href).add_class("link-row");
    if entry.href.starts_with("http") {
        // rel="me" is what identity consumers (Mastodon, IndieAuth) read off an about page.
        anchor = anchor.attr("rel", "me noopener");
    }

    anchor
        .child(
            span()
                .add_class("badge-initial")
                .aria_attr("hidden", "true")
                .text(initial(entry.label)),
        )
        .child(
            span()
                .add_class("link-text")
                .child(span().add_class("link-label").text(entry.label))
                .child(span().add_class("link-note").text(entry.note)),
        )
        .child(
            span()
                .add_class("link-arrow")
                .aria_attr("hidden", "true")
                .text("↗"),
        )
}

/// The strip the browser fills in once the WebAssembly module has rendered the page again.
fn parity_panel() -> Element {
    section()
        .add_class("parity")
        .set_id("parity")
        .child(h2().text("Esta página foi gerada duas vezes"))
        .child(p().text(
            "O mesmo código Rust montou este HTML no build (nativo) e agora de novo, \
             no seu navegador, compilado para WebAssembly. Os dois resultados são \
             comparados byte a byte aqui embaixo.",
        ))
        .child(
            div()
                .add_class("verdict")
                .set_id("parity-verdict")
                .data_attr("state", "pending")
                .attr("role", "status")
                .text("Verificando…"),
        )
        .child(
            dl().add_class("stats")
                .child(stat("stat-bytes", "Bytes de HTML"))
                .child(stat("stat-time", "Render no WebAssembly"))
                .child(stat("stat-wasm", "Tamanho do módulo")),
        )
        .child(
            button_typed("button")
                .add_class("toggle")
                .set_id("toggle-markup")
                .aria_attr("expanded", "false")
                .aria_attr("controls", "markup")
                .text("Ver o markup gerado"),
        )
        .child(
            pre()
                .add_class("markup")
                .set_id("markup")
                .bool_attr("hidden"),
        )
        .child(noscript().child(p().text(
            "A comparação precisa de JavaScript para carregar o módulo WebAssembly. \
             O HTML que você está lendo já foi gerado pelo winged-rust, sem JavaScript \
             nenhum.",
        )))
}

/// One `<dt>`/`<dd>` pair, with the value left for the browser to fill.
fn stat(id: &str, label: &str) -> Element {
    div()
        .child(dt().text(label))
        .child(dd().set_id(id).text("—"))
}

/// The credit line the demo exists to show.
fn page_footer() -> Element {
    footer()
        .set_role("contentinfo")
        .child(
            p().text("Desenvolvido com ")
                .child(link_to(LIBRARY_REPO).text("winged-rust"))
                .child(span().text(" — HTML tipado em Rust, compilado para WebAssembly.")),
        )
        .child(
            p().add_class("colophon")
                .text("Sem framework, sem runtime, sem template. ")
                .child(link_to(DEMO_REPO).text("Código desta página")),
        )
}

/// The capital letter shown in a link's badge.
fn initial(label: &str) -> String {
    label
        .chars()
        .next()
        .map_or_else(String::new, |c| c.to_uppercase().to_string())
}
