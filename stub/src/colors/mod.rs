// playfair-design/src/colors/mod.rs
//! Semantic color system for the Playfair design system
//!
//! This module provides a comprehensive color palette with semantic naming
//! that works across all themes (Light, Dark, Sepia, Ocean, Twilight, Sage, Slate, Ember).

pub mod primitives;
pub mod semantic;

pub use semantic::SemanticColors;

use crate::Theme;
use iced::Color;

/// Convert an iced::Color to a CSS hex string (e.g., `"#f5f0e8"`).
///
/// Ignores the alpha channel — AG Grid and most CSS properties
/// do not support `#rrggbbaa` consistently across WebKit/WebView2.
pub fn color_to_css_hex(color: Color) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
    )
}

impl Theme {
    /// Get semantic colors for this theme
    pub fn colors(&self) -> SemanticColors {
        match self {
            Theme::Light => semantic::light(),
            Theme::Dark => semantic::dark(),
            Theme::Sepia => semantic::sepia(),
            Theme::Ocean => semantic::ocean(),
            Theme::Twilight => semantic::twilight(),
            Theme::Sage => semantic::sage(),
            Theme::Slate => semantic::slate(),
            Theme::Ember => semantic::ember(),
        }
    }

    /// Export this theme's semantic colors as CSS custom properties.
    ///
    /// Keys use the `--pf-` namespace (Playfair). Values are CSS hex
    /// color strings. Consuming applications pass these to a webview
    /// (e.g., Vitrine's `SetTheme` command) so HTML/CSS content can
    /// match the native Iced theme.
    ///
    /// The token names mirror the `SemanticColors` field names with
    /// hyphens replacing underscores.
    pub fn to_css_tokens(&self) -> std::collections::HashMap<String, String> {
        let c = self.colors();
        let mut m = std::collections::HashMap::new();

        // Surface colors
        m.insert(
            "--pf-surface-primary".into(),
            color_to_css_hex(c.surface_primary),
        );
        m.insert(
            "--pf-surface-secondary".into(),
            color_to_css_hex(c.surface_secondary),
        );
        m.insert(
            "--pf-surface-tertiary".into(),
            color_to_css_hex(c.surface_tertiary),
        );
        m.insert(
            "--pf-surface-hover".into(),
            color_to_css_hex(c.surface_hover),
        );
        m.insert(
            "--pf-surface-selected".into(),
            color_to_css_hex(c.surface_selected),
        );
        m.insert(
            "--pf-surface-disabled".into(),
            color_to_css_hex(c.surface_disabled),
        );

        // Text colors
        m.insert("--pf-text-primary".into(), color_to_css_hex(c.text_primary));
        m.insert(
            "--pf-text-secondary".into(),
            color_to_css_hex(c.text_secondary),
        );
        m.insert(
            "--pf-text-tertiary".into(),
            color_to_css_hex(c.text_tertiary),
        );
        m.insert(
            "--pf-text-disabled".into(),
            color_to_css_hex(c.text_disabled),
        );
        m.insert(
            "--pf-text-on-primary".into(),
            color_to_css_hex(c.text_on_primary),
        );

        // Border colors
        m.insert(
            "--pf-border-default".into(),
            color_to_css_hex(c.border_default),
        );
        m.insert("--pf-border-hover".into(), color_to_css_hex(c.border_hover));
        m.insert("--pf-border-focus".into(), color_to_css_hex(c.border_focus));
        m.insert(
            "--pf-border-divider".into(),
            color_to_css_hex(c.border_divider),
        );

        // Action colors
        m.insert(
            "--pf-action-primary".into(),
            color_to_css_hex(c.action_primary),
        );
        m.insert(
            "--pf-action-primary-hover".into(),
            color_to_css_hex(c.action_primary_hover),
        );
        m.insert(
            "--pf-action-primary-active".into(),
            color_to_css_hex(c.action_primary_active),
        );
        m.insert(
            "--pf-action-secondary".into(),
            color_to_css_hex(c.action_secondary),
        );
        m.insert(
            "--pf-action-secondary-hover".into(),
            color_to_css_hex(c.action_secondary_hover),
        );
        m.insert(
            "--pf-action-danger".into(),
            color_to_css_hex(c.action_danger),
        );
        m.insert(
            "--pf-action-danger-hover".into(),
            color_to_css_hex(c.action_danger_hover),
        );
        m.insert(
            "--pf-action-success".into(),
            color_to_css_hex(c.action_success),
        );
        m.insert(
            "--pf-action-warning".into(),
            color_to_css_hex(c.action_warning),
        );

        // State colors
        m.insert("--pf-state-info".into(), color_to_css_hex(c.state_info));
        m.insert(
            "--pf-state-success".into(),
            color_to_css_hex(c.state_success),
        );
        m.insert(
            "--pf-state-warning".into(),
            color_to_css_hex(c.state_warning),
        );
        m.insert("--pf-state-error".into(), color_to_css_hex(c.state_error));

        m
    }
}
