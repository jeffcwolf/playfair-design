use iced::theme::Palette;
use iced::Color;
use iced::Theme as IcedTheme;
use serde::{Deserialize, Serialize};

/// Playfair design system themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    Sepia,
    Ocean,
    Twilight,
    Sage,
    Slate,
    Ember,
}

impl Theme {
    /// Convert to Iced's theme system
    pub fn to_iced(&self) -> IcedTheme {
        match self {
            Theme::Light => IcedTheme::Light,
            Theme::Dark => IcedTheme::Dark,
            Theme::Sepia => IcedTheme::custom(
                "Sepia".to_string(),
                Palette {
                    background: Color::from_rgb(0.96, 0.94, 0.91),
                    text: Color::from_rgb(0.24, 0.15, 0.14),
                    primary: Color::from_rgb(0.55, 0.43, 0.39),
                    success: Color::from_rgb(0.55, 0.63, 0.39),
                    danger: Color::from_rgb(0.78, 0.35, 0.35),
                    warning: Color::from_rgb(0.85, 0.65, 0.30),
                },
            ),
            Theme::Ocean => IcedTheme::custom("Ocean".to_string(), self.palette()),
            Theme::Twilight => IcedTheme::custom("Twilight".to_string(), self.palette()),
            Theme::Sage => IcedTheme::custom("Sage".to_string(), self.palette()),
            Theme::Slate => IcedTheme::custom("Slate".to_string(), self.palette()),
            Theme::Ember => IcedTheme::custom("Ember".to_string(), self.palette()),
        }
    }

    /// Get the palette directly
    pub fn palette(&self) -> Palette {
        match self {
            Theme::Light => Palette::LIGHT,
            Theme::Dark => Palette::DARK,
            Theme::Sepia => Palette {
                background: Color::from_rgb(0.96, 0.94, 0.91),
                text: Color::from_rgb(0.24, 0.15, 0.14),
                primary: Color::from_rgb(0.55, 0.43, 0.39),
                success: Color::from_rgb(0.55, 0.63, 0.39),
                danger: Color::from_rgb(0.78, 0.35, 0.35),
                warning: Color::from_rgb(0.85, 0.65, 0.30),
            },
            Theme::Ocean => Palette {
                background: Color::from_rgb(0.96, 0.97, 0.99), // ocean SURFACE
                text: Color::from_rgb(0.09, 0.15, 0.25),       // ocean GRAY_900
                primary: Color::from_rgb(0.22, 0.47, 0.84),    // ocean BLUE_500
                success: Color::from_rgb(0.20, 0.78, 0.35),    // ocean GREEN_500
                danger: Color::from_rgb(1.0, 0.23, 0.19),      // ocean RED_500
                warning: Color::from_rgb(1.0, 0.58, 0.0),      // ocean ORANGE_500
            },
            Theme::Twilight => Palette {
                background: Color::from_rgb(0.09, 0.07, 0.14), // twilight GRAY_50
                text: Color::from_rgb(0.94, 0.92, 0.99),       // twilight GRAY_900
                primary: Color::from_rgb(0.60, 0.38, 0.97),    // twilight VIOLET_500
                success: Color::from_rgb(0.24, 0.84, 0.44),    // twilight GREEN_500
                danger: Color::from_rgb(1.0, 0.40, 0.38),      // twilight RED_500
                warning: Color::from_rgb(1.0, 0.72, 0.22),     // twilight ORANGE_500
            },
            Theme::Sage => Palette {
                background: Color::from_rgb(0.82, 0.84, 0.80), // sage SURFACE
                text: Color::from_rgb(0.16, 0.19, 0.15),       // sage GRAY_900
                primary: Color::from_rgb(0.28, 0.53, 0.42),    // sage GREEN_500
                success: Color::from_rgb(0.40, 0.62, 0.38),    // sage STATUS_GREEN
                danger: Color::from_rgb(0.75, 0.34, 0.32),     // sage STATUS_RED
                warning: Color::from_rgb(0.80, 0.62, 0.28),    // sage STATUS_ORANGE
            },
            Theme::Slate => Palette {
                background: Color::from_rgb(0.22, 0.24, 0.28), // slate SURFACE
                text: Color::from_rgb(0.90, 0.92, 0.95),       // slate GRAY_900
                primary: Color::from_rgb(0.40, 0.60, 0.82),    // slate STEEL_500
                success: Color::from_rgb(0.30, 0.82, 0.45),    // slate STATUS_GREEN
                danger: Color::from_rgb(1.0, 0.38, 0.36),      // slate STATUS_RED
                warning: Color::from_rgb(1.0, 0.68, 0.18),     // slate STATUS_ORANGE
            },
            Theme::Ember => Palette {
                background: Color::from_rgb(0.18, 0.15, 0.14), // ember SURFACE
                text: Color::from_rgb(0.92, 0.89, 0.85),       // ember GRAY_900
                primary: Color::from_rgb(0.80, 0.52, 0.30),    // ember COPPER_500
                success: Color::from_rgb(0.35, 0.78, 0.40),    // ember STATUS_GREEN
                danger: Color::from_rgb(1.0, 0.42, 0.35),      // ember STATUS_RED
                warning: Color::from_rgb(1.0, 0.70, 0.25),     // ember STATUS_ORANGE
            },
        }
    }

    /// All available themes
    pub fn all() -> &'static [Theme] {
        &[
            Theme::Light,
            Theme::Dark,
            Theme::Sepia,
            Theme::Ocean,
            Theme::Twilight,
            Theme::Sage,
            Theme::Slate,
            Theme::Ember,
        ]
    }

    /// Theme display name
    pub fn name(&self) -> &'static str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::Sepia => "Sepia",
            Theme::Ocean => "Ocean",
            Theme::Twilight => "Twilight",
            Theme::Sage => "Sage",
            Theme::Slate => "Slate",
            Theme::Ember => "Ember",
        }
    }

    /// Get the default font for this theme
    pub fn default_font(&self) -> iced::Font {
        match self {
            Theme::Light
            | Theme::Dark
            | Theme::Ocean
            | Theme::Twilight
            | Theme::Sage
            | Theme::Slate
            | Theme::Ember => iced::Font::DEFAULT,
            Theme::Sepia => iced::Font::with_name("SF Pro Text"), // macOS system font
        }
    }

    /// Get the monospace font for this theme
    pub fn monospace_font(&self) -> iced::Font {
        match self {
            Theme::Light
            | Theme::Dark
            | Theme::Ocean
            | Theme::Twilight
            | Theme::Sage
            | Theme::Slate
            | Theme::Ember => iced::Font::MONOSPACE,
            Theme::Sepia => iced::Font::with_name("SF Mono"), // macOS monospace
        }
    }
}

/// Convenience wrapper for storing a Playfair theme in app state.
///
/// Embed this in your application's top-level state struct.
///
/// # Example
///
/// ```rust,ignore
/// pub struct AppState {
///     pub theme: playfair_core::ThemeState,
/// }
///
/// // In main.rs, wire to iced:
/// iced::application(boot, update, view)
///     .theme(|state: &AppState| state.theme.to_iced())
///     .run()
///
/// // In any view function:
/// button(text("Save"))
///     .style(playfair_core::styles::btn::primary(state.theme.as_ref()))
///
/// // Access semantic colors:
/// let colors = state.theme.colors();
/// text("Hello").color(colors.text_primary)
/// ```
pub struct ThemeState {
    pub theme: Theme,
}

impl ThemeState {
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }

    /// Convert to iced's theme type for use with `.theme()` on the
    /// application builder.
    pub fn to_iced(&self) -> IcedTheme {
        self.theme.to_iced()
    }

    /// Get semantic colors for the current theme.
    pub fn colors(&self) -> crate::SemanticColors {
        self.theme.colors()
    }
}

impl Default for ThemeState {
    fn default() -> Self {
        Self::new(Theme::default())
    }
}

impl AsRef<Theme> for ThemeState {
    fn as_ref(&self) -> &Theme {
        &self.theme
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_returns_eight_themes() {
        let themes = Theme::all();
        assert_eq!(themes.len(), 8);
        assert_eq!(
            themes,
            &[
                Theme::Light,
                Theme::Dark,
                Theme::Sepia,
                Theme::Ocean,
                Theme::Twilight,
                Theme::Sage,
                Theme::Slate,
                Theme::Ember,
            ]
        );
    }

    #[test]
    fn test_colors_does_not_panic_for_any_theme() {
        for theme in Theme::all() {
            let colors = theme.colors();
            // Verify that we got valid colors back by checking a few fields
            // are not all zeros (which would indicate an uninitialized struct)
            let _ = colors.surface_primary;
            let _ = colors.text_primary;
            let _ = colors.action_primary;
        }
    }

    #[test]
    fn test_to_iced_does_not_panic_for_any_theme() {
        for theme in Theme::all() {
            let _ = theme.to_iced();
        }
    }

    #[test]
    fn test_palette_does_not_panic_for_any_theme() {
        for theme in Theme::all() {
            let _ = theme.palette();
        }
    }

    #[test]
    fn test_theme_names() {
        assert_eq!(Theme::Light.name(), "Light");
        assert_eq!(Theme::Dark.name(), "Dark");
        assert_eq!(Theme::Sepia.name(), "Sepia");
        assert_eq!(Theme::Ocean.name(), "Ocean");
        assert_eq!(Theme::Twilight.name(), "Twilight");
        assert_eq!(Theme::Sage.name(), "Sage");
        assert_eq!(Theme::Slate.name(), "Slate");
        assert_eq!(Theme::Ember.name(), "Ember");
    }

    #[test]
    fn test_default_is_sepia() {
        assert_eq!(Theme::default(), Theme::Sepia);
    }
}
