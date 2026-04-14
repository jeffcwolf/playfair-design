// playfair-design/src/styles/container.rs
//! Container component styles for panels, cards, and surfaces
//!
//! Provides semantic container styles that create visual hierarchy
//! through elevation, borders, and background colors.

use crate::borders::{radius, width};
use crate::elevation::shadow;
use crate::Theme;
use iced::widget::container;
use iced::{Background, Border, Color};

/// Card container (elevated surface)
///
/// Use for: Content cards, list items, standalone elements
/// Features: Background, rounded corners, subtle shadow
pub fn card(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = theme.colors();

    move |_iced_theme| container::Style {
        background: Some(Background::Color(colors.surface_secondary)),
        text_color: Some(colors.text_primary),
        border: Border {
            radius: radius::MD.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::sm(),
        snap: true,
    }
}

/// Panel container (bordered surface, no shadow)
///
/// Use for: Side panels, sections, bordered areas
/// Features: Background, border, no shadow
pub fn panel(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = theme.colors();

    move |_iced_theme| container::Style {
        background: Some(Background::Color(colors.surface_secondary)),
        text_color: Some(colors.text_primary),
        border: Border {
            radius: radius::MD.into(),
            width: width::DEFAULT,
            color: colors.border_default,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

/// Modal container (high elevation overlay)
///
/// Use for: Modals, dialogs, important overlays
/// Features: White background, large shadow, rounded corners
pub fn modal(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = theme.colors();

    move |_iced_theme| container::Style {
        background: Some(Background::Color(colors.surface_secondary)),
        text_color: Some(colors.text_primary),
        border: Border {
            radius: radius::LG.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::xl(),
        snap: true,
    }
}

/// Sidebar container (no rounding, subtle border)
///
/// Use for: Navigation panels, sidebars, fixed panels
/// Features: No border radius, subtle border on one side
pub fn sidebar(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = theme.colors();

    move |_iced_theme| container::Style {
        background: Some(Background::Color(colors.surface_primary)),
        text_color: Some(colors.text_primary),
        border: Border {
            radius: 0.0.into(),
            width: width::HAIRLINE,
            color: colors.border_divider,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

/// Section container (subtle background, no border)
///
/// Use for: Grouping related content, form sections
/// Features: Subtle background color, rounded, no shadow
pub fn section(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = theme.colors();

    move |_iced_theme| container::Style {
        background: Some(Background::Color(colors.surface_tertiary)),
        text_color: Some(colors.text_primary),
        border: Border {
            radius: radius::MD.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

/// Inline container (minimal styling)
///
/// Use for: Inline grouping, transparent wrappers
/// Features: Transparent background, no border, no shadow
pub fn inline(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = theme.colors();

    move |_iced_theme| container::Style {
        background: None,
        text_color: Some(colors.text_primary),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

/// Success container (green background for success messages)
///
/// Use for: Success messages, confirmations
/// Features: Green tint, rounded, no shadow
pub fn success(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = theme.colors();

    move |_iced_theme| container::Style {
        background: Some(Background::Color(Color::from_rgba(
            colors.state_success.r,
            colors.state_success.g,
            colors.state_success.b,
            0.1,
        ))),
        text_color: Some(darken(colors.state_success, 0.3)),
        border: Border {
            radius: radius::MD.into(),
            width: width::DEFAULT,
            color: colors.state_success,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

/// Warning container (orange background for warnings)
///
/// Use for: Warning messages, cautions
/// Features: Orange tint, rounded, border
pub fn warning(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = theme.colors();

    move |_iced_theme| container::Style {
        background: Some(Background::Color(Color::from_rgba(
            colors.state_warning.r,
            colors.state_warning.g,
            colors.state_warning.b,
            0.1,
        ))),
        text_color: Some(darken(colors.state_warning, 0.3)),
        border: Border {
            radius: radius::MD.into(),
            width: width::DEFAULT,
            color: colors.state_warning,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

/// Error container (red background for errors)
///
/// Use for: Error messages, validation failures
/// Features: Red tint, rounded, border
pub fn error(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = theme.colors();

    move |_iced_theme| container::Style {
        background: Some(Background::Color(Color::from_rgba(
            colors.state_error.r,
            colors.state_error.g,
            colors.state_error.b,
            0.1,
        ))),
        text_color: Some(darken(colors.state_error, 0.3)),
        border: Border {
            radius: radius::MD.into(),
            width: width::DEFAULT,
            color: colors.state_error,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

/// Info container (blue background for informational messages)
///
/// Use for: Info messages, tips, hints
/// Features: Blue tint, rounded, border
pub fn info(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = theme.colors();

    move |_iced_theme| container::Style {
        background: Some(Background::Color(Color::from_rgba(
            colors.state_info.r,
            colors.state_info.g,
            colors.state_info.b,
            0.1,
        ))),
        text_color: Some(darken(colors.state_info, 0.3)),
        border: Border {
            radius: radius::MD.into(),
            width: width::DEFAULT,
            color: colors.state_info,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

// Helper function to darken a color
fn darken(color: Color, amount: f32) -> Color {
    Color::from_rgba(
        (color.r * (1.0 - amount)).max(0.0),
        (color.g * (1.0 - amount)).max(0.0),
        (color.b * (1.0 - amount)).max(0.0),
        color.a,
    )
}
