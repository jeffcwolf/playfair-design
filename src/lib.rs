//! Playfair Design System
//!
//! Shared design tokens, themes, and UI components for the Scribe suite of applications.
//!
//! # Philosophy
//!
//! Playfair provides a comprehensive design system with:
//! - **Semantic colors** that adapt to themes
//! - **Elevation system** for visual depth
//! - **Component styles** for consistent UI
//! - **Design tokens** for spacing, typography, borders
//!
//! # Usage
//!
//! ```rust,ignore
//! use playfair_core::{Theme, styles};
//!
//! let theme = Theme::Light;
//! let colors = theme.colors();
//!
//! // Use semantic colors
//! let primary_color = colors.action_primary;
//!
//! // Use component styles
//! button(text("Save"))
//!     .style(styles::btn::primary(&theme))
//! ```

pub mod borders;
pub mod colors;
pub mod elevation;
pub mod icons;
pub mod spacing;
pub mod styles;
pub mod theme;
pub mod typography;
pub mod widgets;

/// Raw bytes of the Lucide icon font, suitable for loading with `iced::font::load`.
pub const LUCIDE_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/Lucide.ttf");

/// An [`iced::Font`] descriptor that references the loaded Lucide icon font by name.
pub const LUCIDE_FONT: iced::Font = iced::Font::with_name("Lucide");

pub mod prelude;

// Re-export commonly used items
pub use colors::color_to_css_hex;
pub use colors::SemanticColors;
pub use iced::Theme as IcedTheme;
pub use iced::{Color, Font};
pub use theme::Theme;
pub use theme::ThemeState;
pub use typography::{default_font, monospace_font};
