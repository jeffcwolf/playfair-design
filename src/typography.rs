// playfair-core/src/typography.rs
//! Typography system with scales, weights, and semantic text styles
//!
//! Provides a complete typography foundation for consistent,
//! readable interfaces across all themes.

pub mod scale;
pub mod weights;

use iced::Font;

// Re-export commonly used items
pub use scale::{line_height, size};
pub use weights::{BOLD, LIGHT, MEDIUM, REGULAR, SEMIBOLD};

// Legacy sizes module for backwards compatibility
/// Standard font sizes used across all apps
///
/// **Deprecated**: Use `scale::size` instead for the new typography system
pub mod sizes {
    /// Extra small text (10px)
    pub const XS: f32 = 10.0;

    /// Small text (12px)
    pub const SM: f32 = 12.0;

    /// Base/body text (14px)
    pub const BASE: f32 = 14.0;

    /// Medium text (16px)
    pub const MD: f32 = 16.0;

    /// Large text (18px)
    pub const LG: f32 = 18.0;

    /// Extra large text (20px)
    pub const XL: f32 = 20.0;

    /// Heading sizes
    pub const H1: f32 = 32.0;
    pub const H2: f32 = 28.0;
    pub const H3: f32 = 24.0;
    pub const H4: f32 = 20.0;
    pub const H5: f32 = 18.0;
    pub const H6: f32 = 16.0;
}

/// Standard fonts
pub mod fonts {
    use super::Font;

    /// Default system font
    pub const DEFAULT: Font = Font::DEFAULT;

    /// Monospace font for code
    pub const MONOSPACE: Font = Font::MONOSPACE;

    /// Bold font variant
    pub fn bold() -> Font {
        Font::with_name("Helvetica Neue Bold")
    }

    /// Italic font variant
    pub fn italic() -> Font {
        Font::with_name("Helvetica Neue Italic")
    }
}

/// Get the default font for a given theme
pub fn default_font() -> Font {
    Font::DEFAULT
}

/// Get the monospace font for a given theme
pub fn monospace_font() -> Font {
    Font::MONOSPACE
}
