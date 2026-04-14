// playfair-design/src/styles/aw/card.rs
//! Card component styles for iced_aw with theme-aware variants
//!
//! Provides consistent card styling that maps Playfair's semantic
//! color tokens onto `iced_aw::style::card::Style`.

use crate::borders::radius;
use crate::Theme;
use iced_aw::style::card;
use iced_aw::style::status::Status;

/// Standard card style
///
/// Use for: Content cards, detail panels, and grouped information.
/// Maps to `surface_secondary` background with distinct head/body/foot
/// sections styled according to the current Playfair theme.
///
/// # Example
///
/// ```rust,ignore
/// Card::new(text("Title"), text("Body content"))
///     .style(playfair_core::styles::aw::card::standard(&theme))
/// ```
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, Status) -> card::Style {
    let colors = theme.colors();

    move |_iced_theme, _status| card::Style {
        background: iced::Background::Color(colors.surface_secondary),
        border_radius: radius::MD,
        border_width: 1.0,
        border_color: colors.border_default,
        head_background: iced::Background::Color(colors.surface_tertiary),
        head_text_color: colors.text_primary,
        body_background: iced::Background::Color(colors.surface_secondary),
        body_text_color: colors.text_primary,
        foot_background: iced::Background::Color(colors.surface_secondary),
        foot_text_color: colors.text_secondary,
        close_color: colors.text_secondary,
    }
}
