// playfair-core/src/styles/text.rs
//! Text styling functions for consistent typography
//!
//! Provides semantic text styles that adapt to themes and
//! maintain visual hierarchy across the application.

use crate::typography::{scale, weights};
use crate::Theme;
use iced::widget::text;
use iced::Font;

/// Display text style
///
/// Use for: Hero sections, splash screens
/// Size: 48px, Weight: Light
pub fn display<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::DISPLAY)
        .font(Font {
            weight: weights::LIGHT,
            ..Default::default()
        })
        .color(colors.text_primary)
}

/// H1 heading style
///
/// Use for: Page titles, main headings
/// Size: 38px, Weight: Semibold
pub fn h1<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::H1)
        .font(Font {
            weight: weights::SEMIBOLD,
            ..Default::default()
        })
        .color(colors.text_primary)
}

/// H2 heading style
///
/// Use for: Section titles
/// Size: 30px, Weight: Semibold
pub fn h2<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::H2)
        .font(Font {
            weight: weights::SEMIBOLD,
            ..Default::default()
        })
        .color(colors.text_primary)
}

/// H3 heading style
///
/// Use for: Subsection titles
/// Size: 24px, Weight: Semibold
pub fn h3<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::H3)
        .font(Font {
            weight: weights::SEMIBOLD,
            ..Default::default()
        })
        .color(colors.text_primary)
}

/// H4 heading style
///
/// Use for: Group titles, card headers
/// Size: 20px, Weight: Medium
pub fn h4<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::H4)
        .font(Font {
            weight: weights::MEDIUM,
            ..Default::default()
        })
        .color(colors.text_primary)
}

/// H5 heading style
///
/// Use for: Small headings, list headers
/// Size: 16px, Weight: Medium
pub fn h5<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::H5)
        .font(Font {
            weight: weights::MEDIUM,
            ..Default::default()
        })
        .color(colors.text_primary)
}

/// H6 heading style
///
/// Use for: Tiny headings, overlines
/// Size: 14px, Weight: Medium
pub fn h6<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::H6)
        .font(Font {
            weight: weights::MEDIUM,
            ..Default::default()
        })
        .color(colors.text_primary)
}

/// Body large style
///
/// Use for: Emphasized paragraphs, introductions
/// Size: 16px, Weight: Regular
pub fn body_lg<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::BODY_LG)
        .font(Font {
            weight: weights::REGULAR,
            ..Default::default()
        })
        .color(colors.text_primary)
}

/// Body default style
///
/// Use for: Standard body text, paragraphs
/// Size: 14px, Weight: Regular
pub fn body<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::BODY)
        .font(Font {
            weight: weights::REGULAR,
            ..Default::default()
        })
        .color(colors.text_primary)
}

/// Body small style
///
/// Use for: De-emphasized text, help text
/// Size: 13px, Weight: Regular
pub fn body_sm<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::BODY_SM)
        .font(Font {
            weight: weights::REGULAR,
            ..Default::default()
        })
        .color(colors.text_secondary)
}

/// Caption style
///
/// Use for: Labels, image captions, metadata
/// Size: 12px, Weight: Regular
pub fn caption<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::CAPTION)
        .font(Font {
            weight: weights::REGULAR,
            ..Default::default()
        })
        .color(colors.text_secondary)
}

/// Tiny text style
///
/// Use for: Footnotes, timestamps, fine print
/// Size: 11px, Weight: Regular
pub fn tiny<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::TINY)
        .font(Font {
            weight: weights::REGULAR,
            ..Default::default()
        })
        .color(colors.text_tertiary)
}

/// Code inline style
///
/// Use for: Inline code snippets
/// Size: 13px, Monospace font
pub fn code<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::CODE)
        .font(crate::monospace_font())
        .color(colors.text_primary)
}

/// Emphasized text style
///
/// Use for: Important inline text, highlights
/// Size: 14px, Weight: Medium
pub fn emphasized<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::BODY)
        .font(Font {
            weight: weights::MEDIUM,
            ..Default::default()
        })
        .color(colors.text_primary)
}

/// Muted text style
///
/// Use for: Disabled text, placeholders
/// Size: 14px, Weight: Regular
pub fn muted<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::BODY)
        .font(Font {
            weight: weights::REGULAR,
            ..Default::default()
        })
        .color(colors.text_disabled)
}

/// Link text style
///
/// Use for: Clickable links, interactive text
/// Size: 14px, Weight: Regular, Primary color
pub fn link<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::BODY)
        .font(Font {
            weight: weights::REGULAR,
            ..Default::default()
        })
        .color(colors.action_primary)
}

/// Error text style
///
/// Use for: Error messages, validation errors
/// Size: 13px, Weight: Regular, Error color
pub fn error<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::BODY_SM)
        .font(Font {
            weight: weights::REGULAR,
            ..Default::default()
        })
        .color(colors.state_error)
}

/// Success text style
///
/// Use for: Success messages, confirmations
/// Size: 13px, Weight: Regular, Success color
pub fn success<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::BODY_SM)
        .font(Font {
            weight: weights::REGULAR,
            ..Default::default()
        })
        .color(colors.state_success)
}

/// Warning text style
///
/// Use for: Warning messages, alerts
/// Size: 13px, Weight: Regular, Warning color
pub fn warning<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string())
        .size(scale::size::BODY_SM)
        .font(Font {
            weight: weights::REGULAR,
            ..Default::default()
        })
        .color(colors.state_warning)
}

/// Label text style (uppercase)
///
/// Use for: Form labels, section labels
/// Size: 12px, Weight: Medium, Uppercase
pub fn label<'a>(theme: &Theme, content: impl ToString) -> text::Text<'a> {
    let colors = theme.colors();
    text(content.to_string().to_uppercase())
        .size(scale::size::CAPTION)
        .font(Font {
            weight: weights::MEDIUM,
            ..Default::default()
        })
        .color(colors.text_secondary)
}
