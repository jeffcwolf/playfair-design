// playfair-core/src/styles/toggler.rs
//! Toggler (toggle switch) component styles with theme-aware variants
//!
//! Provides consistent toggle switch styling across all apps with proper
//! on/off, hover, and disabled states.

use crate::colors::SemanticColors;
use crate::Theme;
use iced::widget::toggler;
use iced::{Background, Color};

/// Standard toggler style
///
/// Use for: All boolean toggle switches throughout the application.
/// When toggled on, the background uses `action_primary` and the knob
/// uses `text_on_primary`. When toggled off, the background uses
/// `surface_tertiary` with a `border_default` ring and the knob is
/// `text_tertiary`. Hover and disabled states adjust accordingly.
///
/// # Example
///
/// ```rust,ignore
/// toggler(is_enabled)
///     .on_toggle(Message::Toggled)
///     .style(playfair_core::styles::toggler::standard(&theme))
/// ```
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, toggler::Status) -> toggler::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        toggler::Status::Active { is_toggled } => {
            if is_toggled {
                active_on(&colors)
            } else {
                active_off(&colors)
            }
        }
        toggler::Status::Hovered { is_toggled } => {
            if is_toggled {
                hovered_on(&colors)
            } else {
                hovered_off(&colors)
            }
        }
        toggler::Status::Disabled { is_toggled } => {
            if is_toggled {
                disabled_on(&colors)
            } else {
                disabled_off(&colors)
            }
        }
    }
}

fn active_on(colors: &SemanticColors) -> toggler::Style {
    toggler::Style {
        background: Background::Color(colors.action_primary),
        background_border_width: 0.0,
        background_border_color: Color::TRANSPARENT,
        foreground: Background::Color(colors.text_on_primary),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: Some(colors.text_primary),
        border_radius: None,
        padding_ratio: 0.0,
    }
}

fn active_off(colors: &SemanticColors) -> toggler::Style {
    toggler::Style {
        background: Background::Color(colors.surface_tertiary),
        background_border_width: 1.0,
        background_border_color: colors.border_default,
        foreground: Background::Color(colors.text_tertiary),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: Some(colors.text_primary),
        border_radius: None,
        padding_ratio: 0.0,
    }
}

fn hovered_on(colors: &SemanticColors) -> toggler::Style {
    toggler::Style {
        background: Background::Color(colors.action_primary_hover),
        background_border_width: 0.0,
        background_border_color: Color::TRANSPARENT,
        foreground: Background::Color(colors.text_on_primary),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: Some(colors.text_primary),
        border_radius: None,
        padding_ratio: 0.0,
    }
}

fn hovered_off(colors: &SemanticColors) -> toggler::Style {
    toggler::Style {
        background: Background::Color(colors.surface_hover),
        background_border_width: 1.0,
        background_border_color: colors.border_hover,
        foreground: Background::Color(colors.text_tertiary),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: Some(colors.text_primary),
        border_radius: None,
        padding_ratio: 0.0,
    }
}

fn disabled_on(colors: &SemanticColors) -> toggler::Style {
    toggler::Style {
        background: Background::Color(colors.surface_disabled),
        background_border_width: 0.0,
        background_border_color: Color::TRANSPARENT,
        foreground: Background::Color(colors.text_disabled),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: Some(colors.text_disabled),
        border_radius: None,
        padding_ratio: 0.0,
    }
}

fn disabled_off(colors: &SemanticColors) -> toggler::Style {
    toggler::Style {
        background: Background::Color(colors.surface_disabled),
        background_border_width: 1.0,
        background_border_color: colors.text_disabled,
        foreground: Background::Color(colors.text_disabled),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: Some(colors.text_disabled),
        border_radius: None,
        padding_ratio: 0.0,
    }
}
