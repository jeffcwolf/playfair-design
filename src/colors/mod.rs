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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_to_css_hex_converts_correctly() {
        // Sepia WARM_100: 0.96*255=244.8→0xf4, 0.94*255=239.7→0xef, 0.91*255=232.05→0xe8
        assert_eq!(
            color_to_css_hex(Color::from_rgb(0.96, 0.94, 0.91)),
            "#f4efe8"
        );
        // Pure black
        assert_eq!(color_to_css_hex(Color::BLACK), "#000000");
        // Pure white
        assert_eq!(color_to_css_hex(Color::WHITE), "#ffffff");
    }

    #[test]
    fn sepia_css_tokens_contain_expected_keys() {
        let tokens = Theme::Sepia.to_css_tokens();

        // SemanticColors has 28 fields (6 surface + 5 text + 4 border + 9 action + 4 state)
        assert_eq!(
            tokens.len(),
            28,
            "expected 28 CSS tokens, got {}",
            tokens.len()
        );

        // Verify Sepia surface_primary maps to WARM_100
        assert_eq!(tokens.get("--pf-surface-primary").unwrap(), "#f4efe8");
    }

    #[test]
    fn all_themes_produce_same_token_keys() {
        let reference_keys: std::collections::HashSet<_> =
            Theme::Sepia.to_css_tokens().keys().cloned().collect();

        for theme in Theme::all() {
            let keys: std::collections::HashSet<_> =
                theme.to_css_tokens().keys().cloned().collect();
            assert_eq!(
                keys, reference_keys,
                "Theme {:?} has different CSS token keys than Sepia",
                theme
            );
        }
    }

    #[test]
    fn dark_theme_tokens_differ_from_light() {
        let dark = Theme::Dark.to_css_tokens();
        let light = Theme::Light.to_css_tokens();

        assert_ne!(
            dark.get("--pf-surface-primary"),
            light.get("--pf-surface-primary"),
            "Dark and Light should have different --pf-surface-primary values"
        );
    }
}
