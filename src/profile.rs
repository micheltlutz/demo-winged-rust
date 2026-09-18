//! The only file with personal data in it.
//!
//! Everything the page says about a person lives in [`PROFILE`]. Fork the demo, edit this
//! one constant, and the whole site follows — including the `<title>`, the Open Graph
//! tags and the sitemap.
//!
//! ```
//! use winged_demo::profile::PROFILE;
//! assert!(!PROFILE.links.is_empty());
//! ```

/// One row in the link list.
pub struct Link {
    /// The clickable label, e.g. `"GitHub"`.
    pub label: &'static str,
    /// Where it points. Absolute, including the scheme.
    pub href: &'static str,
    /// The grey line under the label. Say what the reader will find there.
    pub note: &'static str,
}

/// Everything the page renders.
pub struct Profile {
    /// Full name, used as the `<h1>` and in the page title.
    pub name: &'static str,
    /// The `@handle`, rendered under the name and reused as `twitter:site`.
    pub handle: &'static str,
    /// One line that says what this person does. Doubles as the meta description.
    pub headline: &'static str,
    /// A short paragraph. Two or three sentences is the right length for this layout.
    pub bio: &'static str,
    /// Avatar URL.
    pub avatar: &'static str,
    /// Alt text for the avatar — required by `image()` and checked by the a11y audit.
    pub avatar_alt: &'static str,
    /// City and country, shown next to the handle.
    pub location: &'static str,
    /// The links, rendered in this order.
    pub links: &'static [Link],
}

/// The profile this build renders.
pub const PROFILE: Profile = Profile {
    name: "Michel Anderson Lutz Teixeira",
    handle: "@micheltlutz",
    headline: "Engenheiro de software — Swift, Rust e ferramentas que geram HTML.",
    bio: "Autor do Winged-Swift e do winged-rust: a mesma biblioteca de geração de HTML \
          escrita duas vezes, em duas linguagens, com paridade byte a byte verificada por \
          fixtures compartilhadas. Esta página é a prova viva disso.",
    // GitHub serves every account's avatar from this path; no asset to commit.
    avatar: "https://github.com/micheltlutz.png?size=240",
    avatar_alt: "Foto de perfil de Michel Anderson Lutz Teixeira",
    location: "Brasil",
    links: &[
        Link {
            label: "winged-rust",
            href: "https://github.com/micheltlutz/winged-rust",
            note: "A biblioteca que gerou esta página — Rust e WebAssembly",
        },
        Link {
            label: "Winged-Swift",
            href: "https://github.com/micheltlutz/Winged-Swift",
            note: "O original em Swift, do qual o winged-rust é um port",
        },
        Link {
            label: "GitHub",
            href: "https://github.com/micheltlutz",
            note: "Todo o resto do código aberto",
        },
        Link {
            label: "crates.io",
            href: "https://crates.io/crates/winged-rust",
            // TODO: confirmar — o crate ainda não foi publicado (não existe tag v* no repo).
            note: "O crate publicado",
        },
        Link {
            label: "npm",
            href: "https://www.npmjs.com/package/winged-rust",
            // TODO: confirmar — publicado junto com o crate, pelo release.yml.
            note: "O mesmo motor, empacotado como WebAssembly",
        },
        Link {
            label: "LinkedIn",
            // TODO: confirmar a URL do perfil.
            href: "https://www.linkedin.com/in/micheltlutz/",
            note: "Trajetória profissional",
        },
        Link {
            label: "E-mail",
            href: "mailto:michel@micheltlutz.me",
            note: "michel@micheltlutz.me",
        },
    ],
};

/// Where this build will be served from.
///
/// Set `DEMO_SITE_URL` at compile time to point the canonical link and the Open Graph tags
/// somewhere else — each deploy target in the README exports its own.
pub const SITE_URL: &str = match option_env!("DEMO_SITE_URL") {
    Some(url) => url,
    None => "https://micheltlutz.github.io/demo-winged-rust",
};
