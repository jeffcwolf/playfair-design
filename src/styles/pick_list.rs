// playfair-core/src/styles/pick_list.rs
//! Pick list (dropdown select) component styles with theme-aware variants
//!
//! Provides consistent pick list styling across all apps with proper
//! active, hovered, and opened states.

use crate::borders::{radius, width};
use crate::colors::SemanticColors;
use crate::Theme;
use iced::widget::pick_list;
use iced::{Background, Border};

/// Standard pick list style
///
/// Use for: All dropdown select inputs throughout the application.
/// Provides a bordered input appearance in its active state, highlights
/// the border on hover, and applies a focus ring when the list is opened.
///
/// # Example
///
/// ```rust,ignore
/// pick_list(&options, Some(selected), Message::Selected)
///     .style(playfair_core::styles::pick_list::standard(&theme))
/// ```
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, pick_list::Status) -> pick_list::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        pick_list::Status::Active => active(&colors),
        pick_list::Status::Hovered => hovered(&colors),
        pick_list::Status::Opened { .. } => opened(&colors),
    }
}

fn active(colors: &SemanticColors) -> pick_list::Style {
    pick_list::Style {
        text_color: colors.text_primary,
        placeholder_color: colors.text_tertiary,
        handle_color: colors.text_secondary,
        background: Background::Color(colors.surface_secondary),
        border: Border {
            radius: radius::MD.into(),
            width: width::DEFAULT,
            color: colors.border_default,
        },
    }
}

fn hovered(colors: &SemanticColors) -> pick_list::Style {
    pick_list::Style {
        text_color: colors.text_primary,
        placeholder_color: colors.text_tertiary,
        handle_color: colors.text_secondary,
        background: Background::Color(colors.surface_hover),
        border: Border {
            radius: radius::MD.into(),
            width: width::DEFAULT,
            color: colors.border_hover,
        },
    }
}

fn opened(colors: &SemanticColors) -> pick_list::Style {
    pick_list::Style {
        text_color: colors.text_primary,
        placeholder_color: colors.text_tertiary,
        handle_color: colors.text_secondary,
        background: Background::Color(colors.surface_secondary),
        border: Border {
            radius: radius::MD.into(),
            width: width::THICK,
            color: colors.border_focus,
        },
    }
}
