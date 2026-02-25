// playfair-core/src/styles/button.rs
//! Button component styles with semantic variants
//!
//! Provides consistent button styling across all apps with proper
//! hover, active, and disabled states.

use crate::borders::{radius, width};
use crate::colors::SemanticColors;
use crate::elevation::shadow;
use crate::Theme;
use iced::widget::button;
use iced::{Background, Border, Color};

/// Primary action button (most important CTA)
///
/// Use for: Main actions, submit buttons, important CTAs
/// Example: "Save", "Create", "Submit"
pub fn primary(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        button::Status::Active => active_primary(&colors),
        button::Status::Hovered => hovered_primary(&colors),
        button::Status::Pressed => pressed_primary(&colors),
        button::Status::Disabled => disabled(),
    }
}

/// Secondary button (less prominent)
///
/// Use for: Secondary actions, cancel buttons, non-critical actions
/// Example: "Cancel", "Back", "Skip"
pub fn secondary(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        button::Status::Active => active_secondary(&colors),
        button::Status::Hovered => hovered_secondary(&colors),
        button::Status::Pressed => pressed_secondary(&colors),
        button::Status::Disabled => disabled(),
    }
}

/// Danger button (destructive actions)
///
/// Use for: Delete, remove, destructive actions
/// Example: "Delete", "Remove", "Clear All"
pub fn danger(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        button::Status::Active => active_danger(&colors),
        button::Status::Hovered => hovered_danger(&colors),
        button::Status::Pressed => pressed_danger(&colors),
        button::Status::Disabled => disabled(),
    }
}

/// Success button (positive actions)
///
/// Use for: Confirm, approve, success actions
/// Example: "Confirm", "Approve", "Accept"
pub fn success(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        button::Status::Active => active_success(&colors),
        button::Status::Hovered => hovered_success(&colors),
        button::Status::Pressed => pressed_success(&colors),
        button::Status::Disabled => disabled(),
    }
}

/// Ghost button (transparent with border)
///
/// Use for: Tertiary actions, minimal emphasis
/// Example: "Learn More", "Details", navigation
pub fn ghost(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        button::Status::Active => active_ghost(&colors),
        button::Status::Hovered => hovered_ghost(&colors),
        button::Status::Pressed => pressed_ghost(&colors),
        button::Status::Disabled => disabled(),
    }
}

/// Minimal button (text only, no background or border)
///
/// Use for: Links, subtle actions, inline buttons
/// Example: "Show more", "Collapse", inline links
pub fn minimal(theme: &Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = theme.colors();

    move |_iced_theme, status| match status {
        button::Status::Active => active_minimal(&colors),
        button::Status::Hovered => hovered_minimal(&colors),
        button::Status::Pressed => pressed_minimal(&colors),
        button::Status::Disabled => disabled(),
    }
}

// Primary variants
fn active_primary(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.action_primary)),
        text_color: colors.text_on_primary,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

fn hovered_primary(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.action_primary_hover)),
        text_color: colors.text_on_primary,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

fn pressed_primary(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.action_primary_active)),
        text_color: colors.text_on_primary,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

// Secondary variants
fn active_secondary(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.action_secondary)),
        text_color: colors.text_primary,
        border: Border {
            radius: radius::SM.into(),
            width: width::HAIRLINE,
            color: colors.border_default,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

fn hovered_secondary(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.action_secondary_hover)),
        text_color: colors.text_primary,
        border: Border {
            radius: radius::SM.into(),
            width: width::HAIRLINE,
            color: colors.border_hover,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

fn pressed_secondary(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.surface_hover)),
        text_color: colors.text_primary,
        border: Border {
            radius: radius::SM.into(),
            width: width::HAIRLINE,
            color: colors.border_hover,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

// Danger variants
fn active_danger(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.action_danger)),
        text_color: colors.text_on_primary,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

fn hovered_danger(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.action_danger_hover)),
        text_color: colors.text_on_primary,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

fn pressed_danger(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(darken(colors.action_danger_hover, 0.1))),
        text_color: colors.text_on_primary,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

// Success variants
fn active_success(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.action_success)),
        text_color: colors.text_on_primary,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

fn hovered_success(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(darken(colors.action_success, 0.1))),
        text_color: colors.text_on_primary,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

fn pressed_success(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(darken(colors.action_success, 0.2))),
        text_color: colors.text_on_primary,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

// Ghost variants
fn active_ghost(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: None,
        text_color: colors.action_primary,
        border: Border {
            radius: radius::SM.into(),
            width: width::DEFAULT,
            color: colors.action_primary,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

fn hovered_ghost(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.surface_selected)),
        text_color: colors.action_primary_hover,
        border: Border {
            radius: radius::SM.into(),
            width: width::DEFAULT,
            color: colors.action_primary_hover,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

fn pressed_ghost(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.surface_hover)),
        text_color: colors.action_primary_active,
        border: Border {
            radius: radius::SM.into(),
            width: width::DEFAULT,
            color: colors.action_primary_active,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

// Minimal variants
fn active_minimal(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: None,
        text_color: colors.action_primary,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

fn hovered_minimal(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.surface_hover)),
        text_color: colors.action_primary_hover,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

fn pressed_minimal(colors: &SemanticColors) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors.surface_selected)),
        text_color: colors.action_primary_active,
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: shadow::none(),
        snap: true,
    }
}

// Disabled state (same for all variants)
fn disabled() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.94, 0.94, 0.94))),
        text_color: Color::from_rgb(0.7, 0.7, 0.7),
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
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
