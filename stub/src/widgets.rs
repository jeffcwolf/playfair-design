//! Reusable composite widgets built from Playfair design tokens.
//!
//! These are higher-level helpers that assemble multiple Iced widgets
//! into a single `Element`, styled according to the design system.

use crate::styles;
use crate::typography::{scale, weights};
use crate::Theme;
use iced::widget::{button, row, text};
use iced::{Element, Font};

/// Render a horizontal tab bar.
///
/// Each entry in `tabs` is a `(label, tab_value)` pair. The tab matching
/// `active` is rendered with the primary button style; all others use the
/// secondary style. Buttons are laid out in a `row![]` with `spacing(10)`
/// and `padding(10)`.
///
/// # Example
///
/// ```ignore
/// use playfair_core::widgets::tab_bar;
///
/// tab_bar(
///     &[("Import", View::Import), ("Export", View::Export)],
///     state.current_view,
///     Message::SwitchView,
///     theme,
/// )
/// ```
pub fn tab_bar<'a, Tab, Message>(
    tabs: &[(&'a str, Tab)],
    active: Tab,
    on_select: impl Fn(Tab) -> Message + 'static,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    Tab: Clone + PartialEq + 'static,
    Message: Clone + 'static,
{
    let mut tab_row = row![].spacing(2);

    for (label, tab) in tabs {
        let msg = on_select(tab.clone());
        // Use a plain text widget (no explicit color) so the button
        // style's text_color controls contrast on both primary and
        // secondary backgrounds.
        let tab_label = text(*label).size(scale::size::BODY_SM).font(Font {
            weight: weights::MEDIUM,
            ..Default::default()
        });
        let btn = button(tab_label).on_press(msg).padding([4, 10]);

        tab_row = tab_row.push(if *tab == active {
            btn.style(styles::btn::primary(theme))
        } else {
            btn.style(styles::btn::secondary(theme))
        });
    }

    tab_row.into()
}
