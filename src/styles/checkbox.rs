// playfair-core/src/styles/checkbox.rs
//! Checkbox component styles with theme-aware variants
//!
//! Provides consistent checkbox styling across all apps with proper
//! checked, unchecked, hover, and disabled states.

use crate::borders::{radius, width};
use crate::colors::SemanticColors;
use crate::Theme;
use iced::widget::checkbox;
use iced::{Background, Border, Color};

/// Standard checkbox style
///
/// Uses the theme's action_primary color for the checked state and
/// adapts to light, dark, and sepia themes.
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, checkbox::Status) -> checkbox::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        checkbox::Status::Active { is_checked } => {
            if is_checked {
                checked(&colors)
            } else {
                unchecked(&colors)
            }
        }
        checkbox::Status::Hovered { is_checked } => {
            if is_checked {
                checked_hovered(&colors)
            } else {
                unchecked_hovered(&colors)
            }
        }
        checkbox::Status::Disabled { is_checked } => {
            if is_checked {
                checked_disabled(&colors)
            } else {
                unchecked_disabled(&colors)
            }
        }
    }
}

fn checked(colors: &SemanticColors) -> checkbox::Style {
    checkbox::Style {
        background: Background::Color(colors.action_primary),
        icon_color: colors.text_on_primary,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        text_color: Some(colors.text_primary),
    }
}

fn checked_hovered(colors: &SemanticColors) -> checkbox::Style {
    checkbox::Style {
        background: Background::Color(colors.action_primary_hover),
        icon_color: colors.text_on_primary,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        text_color: Some(colors.text_primary),
    }
}

fn unchecked(colors: &SemanticColors) -> checkbox::Style {
    checkbox::Style {
        background: Background::Color(colors.surface_secondary),
        icon_color: Color::TRANSPARENT,
        border: Border {
            radius: radius::SM.into(),
            width: width::DEFAULT,
            color: colors.border_default,
        },
        text_color: Some(colors.text_primary),
    }
}

fn unchecked_hovered(colors: &SemanticColors) -> checkbox::Style {
    checkbox::Style {
        background: Background::Color(colors.surface_hover),
        icon_color: Color::TRANSPARENT,
        border: Border {
            radius: radius::SM.into(),
            width: width::DEFAULT,
            color: colors.border_hover,
        },
        text_color: Some(colors.text_primary),
    }
}

fn checked_disabled(colors: &SemanticColors) -> checkbox::Style {
    checkbox::Style {
        background: Background::Color(colors.surface_disabled),
        icon_color: colors.text_disabled,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        text_color: Some(colors.text_disabled),
    }
}

fn unchecked_disabled(colors: &SemanticColors) -> checkbox::Style {
    checkbox::Style {
        background: Background::Color(colors.surface_disabled),
        icon_color: Color::TRANSPARENT,
        border: Border {
            radius: radius::SM.into(),
            width: width::DEFAULT,
            color: colors.border_default,
        },
        text_color: Some(colors.text_disabled),
    }
}
