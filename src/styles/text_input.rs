// playfair-core/src/styles/text_input.rs
//! Text input field styles
//!
//! Provides consistent styling for input fields with proper
//! focus, hover, and error states.

use crate::borders::{radius, width};
use crate::colors::SemanticColors;
use crate::Theme;
use iced::widget::text_input;
use iced::{Background, Border, Color};

/// Default text input style
///
/// Use for: Standard input fields
/// Features: Border, focus ring, proper states
pub fn default(theme: &Theme) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        text_input::Status::Active => active(&colors),
        text_input::Status::Hovered => hovered(&colors),
        text_input::Status::Focused { .. } => focused(&colors),
        text_input::Status::Disabled => disabled(&colors),
    }
}

/// Borderless text input (minimal styling)
///
/// Use for: Inline editing, minimal forms
/// Features: No border, subtle background
pub fn borderless(theme: &Theme) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        text_input::Status::Active => text_input::Style {
            background: Background::Color(colors.surface_tertiary),
            border: Border {
                radius: radius::SM.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            icon: colors.text_tertiary,
            placeholder: colors.text_tertiary,
            value: colors.text_primary,
            selection: colors.surface_selected,
        },
        text_input::Status::Hovered => text_input::Style {
            background: Background::Color(colors.surface_hover),
            border: Border {
                radius: radius::SM.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            icon: colors.text_secondary,
            placeholder: colors.text_tertiary,
            value: colors.text_primary,
            selection: colors.surface_selected,
        },
        text_input::Status::Focused { .. } => text_input::Style {
            background: Background::Color(colors.surface_secondary),
            border: Border {
                radius: radius::SM.into(),
                width: width::THICK,
                color: colors.border_focus,
            },
            icon: colors.text_primary,
            placeholder: colors.text_tertiary,
            value: colors.text_primary,
            selection: colors.surface_selected,
        },
        text_input::Status::Disabled => disabled(&colors),
    }
}

// State implementations
fn active(colors: &SemanticColors) -> text_input::Style {
    text_input::Style {
        background: Background::Color(colors.surface_secondary),
        border: Border {
            radius: radius::MD.into(),
            width: width::DEFAULT,
            color: colors.border_default,
        },
        icon: colors.text_tertiary,
        placeholder: colors.text_tertiary,
        value: colors.text_primary,
        selection: colors.surface_selected,
    }
}

fn hovered(colors: &SemanticColors) -> text_input::Style {
    text_input::Style {
        background: Background::Color(colors.surface_secondary),
        border: Border {
            radius: radius::MD.into(),
            width: width::DEFAULT,
            color: colors.border_hover,
        },
        icon: colors.text_secondary,
        placeholder: colors.text_tertiary,
        value: colors.text_primary,
        selection: colors.surface_selected,
    }
}

fn focused(colors: &SemanticColors) -> text_input::Style {
    text_input::Style {
        background: Background::Color(colors.surface_secondary),
        border: Border {
            radius: radius::MD.into(),
            width: width::THICK,
            color: colors.border_focus,
        },
        icon: colors.action_primary,
        placeholder: colors.text_tertiary,
        value: colors.text_primary,
        selection: colors.surface_selected,
    }
}

fn disabled(colors: &SemanticColors) -> text_input::Style {
    text_input::Style {
        background: Background::Color(colors.surface_disabled),
        border: Border {
            radius: radius::MD.into(),
            width: width::DEFAULT,
            color: colors.border_default,
        },
        icon: colors.text_disabled,
        placeholder: colors.text_disabled,
        value: colors.text_disabled,
        selection: Color::TRANSPARENT,
    }
}
