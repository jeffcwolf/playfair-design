// playfair-core/src/styles/radio.rs
//! Radio button component styles with theme-aware variants
//!
//! Provides consistent radio button styling across all apps with proper
//! selected, unselected, and hover states that adapt to the current theme.

use crate::colors::SemanticColors;
use crate::Theme;
use iced::widget::radio;
use iced::{Background, Color};

/// Standard radio button style
///
/// Use for: All radio button groups throughout the application.
/// Renders a circular radio button that uses the theme's `action_primary`
/// color for the selected dot and ring, with `surface_secondary` as the
/// background. Hover states lighten the border to `border_hover`.
///
/// # Example
///
/// ```rust,ignore
/// radio("Option A", Choice::A, Some(selected), Message::Selected)
///     .style(playfair_core::styles::radio::standard(&theme))
/// ```
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, radio::Status) -> radio::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        radio::Status::Active { is_selected } => {
            if is_selected {
                selected_active(&colors)
            } else {
                unselected_active(&colors)
            }
        }
        radio::Status::Hovered { is_selected } => {
            if is_selected {
                selected_hovered(&colors)
            } else {
                unselected_hovered(&colors)
            }
        }
    }
}

fn selected_active(colors: &SemanticColors) -> radio::Style {
    radio::Style {
        background: Background::Color(colors.surface_secondary),
        dot_color: colors.action_primary,
        border_width: 1.5,
        border_color: colors.action_primary,
        text_color: Some(colors.text_primary),
    }
}

fn unselected_active(colors: &SemanticColors) -> radio::Style {
    radio::Style {
        background: Background::Color(colors.surface_secondary),
        dot_color: Color::TRANSPARENT,
        border_width: 1.5,
        border_color: colors.border_default,
        text_color: Some(colors.text_primary),
    }
}

fn selected_hovered(colors: &SemanticColors) -> radio::Style {
    radio::Style {
        background: Background::Color(colors.surface_secondary),
        dot_color: colors.action_primary,
        border_width: 1.5,
        border_color: colors.action_primary,
        text_color: Some(colors.text_primary),
    }
}

fn unselected_hovered(colors: &SemanticColors) -> radio::Style {
    radio::Style {
        background: Background::Color(colors.surface_secondary),
        dot_color: Color::TRANSPARENT,
        border_width: 1.5,
        border_color: colors.border_hover,
        text_color: Some(colors.text_primary),
    }
}
