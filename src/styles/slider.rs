// playfair-core/src/styles/slider.rs
//! Slider component styles with theme-aware variants
//!
//! Provides consistent slider styling across all apps with proper
//! active, hovered, and dragged states.

use crate::borders::radius;
use crate::colors::SemanticColors;
use crate::Theme;
use iced::widget::slider;
use iced::{Background, Border, Color};

/// Standard slider style
///
/// Use for: All range-slider inputs throughout the application. Renders
/// a thin rail with the filled portion in `action_primary` and the
/// unfilled portion in `surface_tertiary`. The circular handle matches
/// `action_primary` and has no border.
///
/// # Example
///
/// ```rust,ignore
/// slider(0.0..=100.0, value, Message::SliderChanged)
///     .style(playfair_core::styles::slider::standard(&theme))
/// ```
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, slider::Status) -> slider::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        slider::Status::Active => active(&colors),
        slider::Status::Hovered => hovered(&colors),
        slider::Status::Dragged => dragged(&colors),
    }
}

fn active(colors: &SemanticColors) -> slider::Style {
    slider::Style {
        rail: slider::Rail {
            backgrounds: (
                Background::Color(colors.action_primary),
                Background::Color(colors.surface_tertiary),
            ),
            width: 4.0,
            border: Border {
                radius: radius::FULL.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 8.0 },
            background: Background::Color(colors.action_primary),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
    }
}

fn hovered(colors: &SemanticColors) -> slider::Style {
    slider::Style {
        rail: slider::Rail {
            backgrounds: (
                Background::Color(colors.action_primary),
                Background::Color(colors.surface_tertiary),
            ),
            width: 4.0,
            border: Border {
                radius: radius::FULL.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 8.0 },
            background: Background::Color(colors.action_primary_hover),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
    }
}

fn dragged(colors: &SemanticColors) -> slider::Style {
    slider::Style {
        rail: slider::Rail {
            backgrounds: (
                Background::Color(colors.action_primary),
                Background::Color(colors.surface_tertiary),
            ),
            width: 4.0,
            border: Border {
                radius: radius::FULL.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 8.0 },
            background: Background::Color(colors.action_primary_active),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
    }
}
