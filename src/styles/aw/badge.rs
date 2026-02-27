// playfair-core/src/styles/aw/badge.rs
//! Badge component styles for iced_aw with theme-aware variants
//!
//! Provides consistent badge styling that maps Playfair's semantic
//! color tokens onto `iced_aw::style::badge::Style`.

use crate::borders::radius;
use crate::Theme;
use iced_aw::style::badge;
use iced_aw::style::status::Status;

/// Standard badge style
///
/// Use for: Status indicators, notification counts, tags, and labels
/// that need to stand out. Maps to `surface_secondary` background with
/// `text_primary` text and a subtle border.
///
/// # Example
///
/// ```rust,ignore
/// Badge::new(text("3"))
///     .style(playfair_core::styles::aw::badge::standard(&theme))
/// ```
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, Status) -> badge::Style {
    let colors = theme.colors();

    move |_iced_theme, status| {
        let base = badge::Style {
            background: iced::Background::Color(colors.surface_secondary),
            border_radius: Some(radius::PILL),
            border_width: 1.0,
            border_color: Some(colors.border_default),
            text_color: colors.text_primary,
        };

        match status {
            Status::Disabled => badge::Style {
                background: iced::Background::Color(colors.surface_disabled),
                text_color: colors.text_disabled,
                border_color: Some(colors.border_default),
                ..base
            },
            _ => base,
        }
    }
}
