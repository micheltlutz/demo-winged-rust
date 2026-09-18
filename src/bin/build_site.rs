//! Renders the page natively and writes `dist/`.
//!
//! Run from the repository root: `cargo run --bin build-site`.
//!
//! The a11y audit is a gate, not a report — a missing `alt` or an unlabelled link fails
//! the build rather than shipping.

use std::io;
use std::process::ExitCode;

use winged_demo::page::document;
use winged_demo::profile::SITE_URL;
use winged_rust::accessibility::audit;
use winged_rust::prelude::Node;
use winged_rust::sitemap::{SitemapGenerator, SitemapUrl};
use winged_rust::ssg::StaticSiteGenerator;

fn main() -> ExitCode {
    let page = document();

    let issues = audit(&Node::from(page.root()));
    if !issues.is_empty() {
        for issue in &issues {
            eprintln!("a11y: <{}> — {}", issue.tag, issue.message);
        }
        eprintln!(
            "{} problema(s) de acessibilidade — nada foi gravado.",
            issues.len()
        );
        return ExitCode::FAILURE;
    }

    match write_site(&page.render()) {
        Ok(bytes) => {
            println!("dist/index.html — {bytes} bytes");
            println!("dist/parity/native.txt — a mesma página, para o browser comparar");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("falha ao gravar dist/: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Writes every file the deploy needs, and returns the size of the page.
///
/// `index.html` and `parity/native.txt` are written from the *same* string rather than
/// rendered twice — the file the browser compares against has to be the file it is
/// looking at, byte for byte.
fn write_site(markup: &str) -> io::Result<usize> {
    let site = StaticSiteGenerator::new("dist");
    site.clean(true)?;

    site.write_file(markup, "index.html")?;
    site.write_file(markup, "parity/native.txt")?;
    site.copy_asset("web/boot.js", "boot.js")?;

    site.write_file(
        &SitemapGenerator::generate(&[SitemapUrl::new(format!("{SITE_URL}/"))
            .changefreq("monthly")
            .priority(1.0)]),
        "sitemap.xml",
    )?;

    site.write_file(
        &format!("User-agent: *\nAllow: /\nSitemap: {SITE_URL}/sitemap.xml\n"),
        "robots.txt",
    )?;

    Ok(markup.len())
}
