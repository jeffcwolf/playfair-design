// playfair-design/src/styles/aw/tab_bar.rs
//! Tab bar component styles for iced_aw with theme-aware variants
//!
//! Provides consistent tab bar styling that maps Playfair's semantic
//! color tokens onto `iced_aw::style::tab_bar::Style`.

use crate::borders::radius;
use crate::Theme;
use iced_aw::style::status::Status;
use iced_aw::style::tab_bar;

/// Standard tab bar style
///
/// Use for: Tab navigation where the active tab should be visually
/// prominent. Active tabs use `action_primary` as their background;
/// inactive tabs use `surface_tertiary` for a subtle, receded look.
/// Disabled tabs are further muted.
///
/// # Example
///
/// ```rust,ignore
/// TabBar::new(active_tab, Message::TabSelected)
///     .push(TabLabel::Text("Tab 1".into()))
///     .push(TabLabel::Text("Tab 2".into()))
///     .style(playfair_core::styles::aw::tab_bar::standard(&theme))
/// ```
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, Status) -> tab_bar::Style {
    let colors = theme.colors();

    move |_iced_theme, status| {
        let base = tab_bar::Style {
            background: None,
            border_color: None,
            border_width: 0.0,
            tab_border_radius: radius::SM.into(),
            tab_label_background: iced::Background::Color(colors.surface_tertiary),
            tab_label_border_color: colors.border_default,
            tab_label_border_width: 1.0,
            icon_color: colors.text_secondary,
            icon_background: None,
            icon_border_radius: radius::SM.into(),
            text_color: colors.text_primary,
        };

        match status {
            Status::Active | Status::Focused | Status::Pressed | Status::Selected => {
                tab_bar::Style {
                    tab_label_background: iced::Background::Color(colors.action_primary),
                    tab_label_border_color: colors.action_primary,
                    icon_color: colors.text_on_primary,
                    text_color: colors.text_on_primary,
                    ..base
                }
            }
            Status::Hovered => tab_bar::Style {
                tab_label_background: iced::Background::Color(colors.action_primary_hover),
                tab_label_border_color: colors.action_primary_hover,
                icon_color: colors.text_on_primary,
                text_color: colors.text_on_primary,
                ..base
            },
            Status::Disabled => tab_bar::Style {
                tab_label_background: iced::Background::Color(colors.surface_disabled),
                tab_label_border_color: colors.border_default,
                icon_color: colors.text_disabled,
                text_color: colors.text_disabled,
                ..base
            },
        }
    }
}
