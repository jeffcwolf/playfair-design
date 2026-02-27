// playfair-core/src/styles/progress_bar.rs
//! Progress bar component styles with theme-aware variants
//!
//! Provides consistent progress bar styling across all apps with
//! semantic color variants for different contexts.

use crate::borders::radius;
use crate::Theme;
use iced::widget::progress_bar;
use iced::{Background, Border, Color};

/// Standard progress bar style
///
/// Use for: General progress indicators, file uploads, loading states.
/// Uses the theme's `action_primary` color for the filled bar and
/// `surface_tertiary` for the unfilled background track.
///
/// # Example
///
/// ```rust,ignore
/// progress_bar(0.0..=100.0, progress_value)
///     .style(playfair_core::styles::progress_bar::standard(&theme))
/// ```
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    let colors = theme.colors();

    move |_iced_theme| progress_bar::Style {
        background: Background::Color(colors.surface_tertiary),
        bar: Background::Color(colors.action_primary),
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
    }
}

/// Subtle progress bar style
///
/// Use for: Secondary or less prominent progress indicators where the
/// primary color would be too attention-grabbing. Uses `action_secondary`
/// for the filled bar.
///
/// # Example
///
/// ```rust,ignore
/// progress_bar(0.0..=100.0, progress_value)
///     .style(playfair_core::styles::progress_bar::subtle(&theme))
/// ```
pub fn subtle(theme: &Theme) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    let colors = theme.colors();

    move |_iced_theme| progress_bar::Style {
        background: Background::Color(colors.surface_tertiary),
        bar: Background::Color(colors.action_secondary),
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
    }
}

/// Danger progress bar style
///
/// Use for: Progress indicators that represent destructive operations,
/// error states, or resource consumption nearing limits (e.g. disk usage).
/// Uses `action_danger` for the filled bar.
///
/// # Example
///
/// ```rust,ignore
/// progress_bar(0.0..=100.0, disk_usage_percent)
///     .style(playfair_core::styles::progress_bar::danger(&theme))
/// ```
pub fn danger(theme: &Theme) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    let colors = theme.colors();

    move |_iced_theme| progress_bar::Style {
        background: Background::Color(colors.surface_tertiary),
        bar: Background::Color(colors.action_danger),
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
    }
}
