// playfair-core/src/styles/aw/number_input.rs
//! Number input component styles for iced_aw with theme-aware variants
//!
//! Provides consistent number input styling that maps Playfair's semantic
//! color tokens onto `iced_aw::style::number_input::Style`.

use crate::Theme;
use iced_aw::style::number_input;
use iced_aw::style::status::Status;

/// Standard number input style
///
/// Use for: Numeric fields with increment/decrement buttons.
/// The arrow buttons use `action_primary` as background with
/// `text_on_primary` icons. Disabled state fades the controls.
///
/// # Example
///
/// ```rust,ignore
/// NumberInput::new(value, 0..=100, Message::Changed)
///     .style(playfair_core::styles::aw::number_input::standard(&theme))
/// ```
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, Status) -> number_input::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        Status::Disabled => number_input::Style {
            button_background: Some(iced::Background::Color(colors.surface_disabled)),
            icon_color: colors.text_disabled,
        },
        _ => number_input::Style {
            button_background: Some(iced::Background::Color(colors.action_primary)),
            icon_color: colors.text_on_primary,
        },
    }
}
