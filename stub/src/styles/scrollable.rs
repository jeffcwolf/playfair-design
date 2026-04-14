// playfair-design/src/styles/scrollable.rs
//! Scrollable container component styles with theme-aware variants
//!
//! Provides consistent scrollbar styling across all apps with proper
//! active, hovered, and dragged states.

use crate::borders::radius;
use crate::colors::SemanticColors;
use crate::elevation::shadow;
use crate::Theme;
use iced::widget::{container, scrollable};
use iced::{Background, Border, Color, Shadow};

/// Standard scrollable style
///
/// Use for: Most scrollable regions in the application. Renders a
/// transparent container with `surface_tertiary` rail backgrounds and
/// medium-width scrollbar handles. The scroller handle changes color
/// on hover and drag to provide interactive feedback.
///
/// # Example
///
/// ```rust,ignore
/// scrollable(content)
///     .style(playfair_core::styles::scrollable::standard(&theme))
/// ```
pub fn standard(theme: &Theme) -> impl Fn(&iced::Theme, scrollable::Status) -> scrollable::Style {
    let colors = theme.colors();

    move |_iced_theme, status| {
        let rail = |scroller_color: Color| scrollable::Rail {
            background: Some(Background::Color(colors.surface_tertiary)),
            border: Border {
                radius: radius::FULL.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            scroller: scrollable::Scroller {
                background: Background::Color(scroller_color),
                border: Border {
                    radius: radius::FULL.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
            },
        };

        let (v_rail, h_rail) = match status {
            scrollable::Status::Active { .. } => {
                let r = rail(colors.text_tertiary);
                (r, r)
            }
            scrollable::Status::Hovered { .. } | scrollable::Status::Dragged { .. } => {
                let r = rail(colors.text_secondary);
                (r, r)
            }
        };

        scrollable::Style {
            container: container::Style {
                background: None,
                text_color: None,
                border: Border {
                    radius: 0.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: shadow::none(),
                snap: false,
            },
            vertical_rail: v_rail,
            horizontal_rail: h_rail,
            gap: None,
            auto_scroll: auto_scroll_default(&colors),
        }
    }
}

/// Thin scrollable style
///
/// Use for: Compact scrollable areas such as sidebars, narrow panels,
/// or nested scrollable lists where a full-width scrollbar would be
/// visually heavy. Behaves identically to [`standard`] but renders a
/// narrower scroller track. Set the scrollbar width on the widget
/// itself (e.g. `.scrollbar_width(4)`) to complement this style.
///
/// # Example
///
/// ```rust,ignore
/// scrollable(content)
///     .scrollbar_width(4)
///     .style(playfair_core::styles::scrollable::thin(&theme))
/// ```
pub fn thin(theme: &Theme) -> impl Fn(&iced::Theme, scrollable::Status) -> scrollable::Style {
    let colors = theme.colors();

    move |_iced_theme, status| {
        let rail = |scroller_color: Color| scrollable::Rail {
            background: Some(Background::Color(colors.surface_tertiary)),
            border: Border {
                radius: radius::FULL.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            scroller: scrollable::Scroller {
                background: Background::Color(scroller_color),
                border: Border {
                    radius: radius::FULL.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
            },
        };

        let (v_rail, h_rail) = match status {
            scrollable::Status::Active { .. } => {
                let r = rail(colors.text_tertiary);
                (r, r)
            }
            scrollable::Status::Hovered { .. } | scrollable::Status::Dragged { .. } => {
                let r = rail(colors.text_secondary);
                (r, r)
            }
        };

        scrollable::Style {
            container: container::Style {
                background: None,
                text_color: None,
                border: Border {
                    radius: 0.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: shadow::none(),
                snap: false,
            },
            vertical_rail: v_rail,
            horizontal_rail: h_rail,
            gap: None,
            auto_scroll: auto_scroll_default(&colors),
        }
    }
}

fn auto_scroll_default(colors: &SemanticColors) -> scrollable::AutoScroll {
    scrollable::AutoScroll {
        background: Background::Color(colors.surface_secondary),
        border: Border {
            radius: radius::SM.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: iced::Vector::ZERO,
            blur_radius: 0.0,
        },
        icon: colors.text_secondary,
    }
}
