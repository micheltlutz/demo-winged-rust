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
    /// A line worth pulling out of the bio and setting in a `<blockquote>`.
    pub quote: &'static str,
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
///
/// Content taken from <https://micheltlutz.me>.
pub const PROFILE: Profile = Profile {
    name: "Michel Lütz",
    handle: "@micheltlutz",
    headline: "Software Architect and AI Engineer",
    bio: "Com mais de 18 anos em engenharia de software, trabalho na interseção entre \
          arquitetura mobile, estratégia de engenharia e adoção responsável de GenAI. \
          Software de missão crítica que continua manutenível depois que eu saio da sala.",
    quote: "Arquitetura não é escolher a tecnologia mais nova. É garantir que, daqui a \
            cinco anos, alguém que nunca conversou comigo consiga entender por que a \
            decisão foi aquela, e mudá-la sem medo.",
    // GitHub serves every account's avatar from this path; no asset to commit.
    avatar: "https://github.com/micheltlutz.png?size=240",
    avatar_alt: "Foto de perfil de Michel Lütz",
    location: "Porto Alegre, RS · remote-first",
    links: &[
        Link {
            label: "micheltlutz.me",
            href: "https://micheltlutz.me",
            note: "Arquitetura mobile, artigos e palestras",
        },
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
            label: "LinkedIn",
            href: "https://www.linkedin.com/in/michellutz/",
            note: "Trajetória profissional",
        },
        Link {
            label: "YouTube",
            href: "https://www.youtube.com/channel/UCfRIPuJSNaW2ZXUJWVpNpUg",
            note: "Palestras e conteúdo técnico",
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
/// The default is the public home of the demo — every mirror points its canonical link
/// here. Set `DEMO_SITE_URL` at compile time to override it, which is what the deploy
/// targets in the README do when they need to name themselves instead.
pub const SITE_URL: &str = match option_env!("DEMO_SITE_URL") {
    Some(url) => url,
    None => "https://demo-winged-rust.micheltlutz.me",
};
