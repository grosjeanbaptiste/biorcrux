//! The design tokens of `biorcrux-design` as terminal colours, plus ratatui re-exported.
//!
//! Colours are read from the stylesheet (the single source of truth), never copied here.

pub use ratatui;

use biorcrux_design::STYLESHEET;
use ratatui::style::{Color, Style};

/// Which colour scheme of the stylesheet to read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scheme {
    Light,
    Dark,
}

/// Every design token as a terminal colour.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Palette {
    pub bg: Color,
    pub surface: Color,
    pub fg: Color,
    pub muted: Color,
    pub dim: Color,
    pub border: Color,
    pub accent: Color,
    pub on_accent: Color,
    pub ok: Color,
    pub warn: Color,
    pub danger: Color,
}

impl Palette {
    /// The palette of `scheme`; `None` only if the stylesheet lacks a token.
    #[must_use]
    pub fn new(scheme: Scheme) -> Option<Self> {
        Self::from_css(STYLESHEET, scheme)
    }

    fn from_css(css: &str, scheme: Scheme) -> Option<Self> {
        let marker = match scheme {
            Scheme::Light => ":root {",
            Scheme::Dark => ":root:where([data-scheme=\"dark\"]",
        };
        let block = block_after(css, marker)?;
        let get = |name: &str| token(block, name);
        Some(Self {
            bg: get("bg")?,
            surface: get("surface")?,
            fg: get("fg")?,
            muted: get("muted")?,
            dim: get("dim")?,
            border: get("border")?,
            accent: get("accent")?,
            on_accent: get("on-accent")?,
            ok: get("ok")?,
            warn: get("warn")?,
            danger: get("danger")?,
        })
    }

    /// Text on the page background.
    #[must_use]
    pub fn base(&self) -> Style {
        Style::new().fg(self.fg).bg(self.bg)
    }

    /// A selected or primary element.
    #[must_use]
    pub fn accent(&self) -> Style {
        Style::new().fg(self.on_accent).bg(self.accent)
    }
}

/// The text between the first `{` after `marker` and its `}`.
fn block_after<'a>(css: &'a str, marker: &str) -> Option<&'a str> {
    let rest = &css[css.find(marker)?..];
    let body = &rest[rest.find('{')? + 1..];
    Some(&body[..body.find('}')?])
}

/// The `#rrggbb` value of `--name` in `block`.
fn token(block: &str, name: &str) -> Option<Color> {
    let key = format!("--{name}:");
    let line = block.lines().find(|l| l.trim_start().starts_with(&key))?;
    let hex = line.split(':').nth(1)?.trim().trim_end_matches(';').trim();
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let n = u32::from_str_radix(hex, 16).ok()?;
    Some(Color::Rgb((n >> 16) as u8, (n >> 8) as u8, n as u8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_light_palette_reads_the_stylesheet() {
        let p = Palette::new(Scheme::Light).expect("tokens");
        assert_eq!(p.accent, Color::Rgb(0x4f, 0x6d, 0xf5));
        assert_eq!(p.bg, Color::Rgb(0xf7, 0xf7, 0xf8));
    }

    #[test]
    fn the_dark_palette_reads_the_dark_block() {
        let p = Palette::new(Scheme::Dark).expect("tokens");
        assert_eq!(p.accent, Color::Rgb(0x7c, 0x93, 0xf7));
        assert_eq!(p.surface, Color::Rgb(0x13, 0x13, 0x16));
    }

    #[test]
    fn every_design_token_has_a_colour_in_both_schemes() {
        for scheme in [Scheme::Light, Scheme::Dark] {
            assert!(Palette::new(scheme).is_some());
        }
        assert_eq!(biorcrux_design::TOKENS.len(), 11);
    }

    #[test]
    fn a_missing_token_gives_no_palette() {
        assert_eq!(Palette::from_css(":root { --bg: #000000; }", Scheme::Light), None);
    }

    #[test]
    fn the_accent_style_puts_on_accent_text_on_the_accent() {
        let p = Palette::new(Scheme::Light).expect("tokens");
        assert_eq!(p.accent(), Style::new().fg(p.on_accent).bg(p.accent));
    }
}
