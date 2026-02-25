// playfair-core/src/colors/mod.rs
//! Semantic color system for the Playfair design system
//!
//! This module provides a comprehensive color palette with semantic naming
//! that works across all themes (Light, Dark, Sepia).

pub mod primitives;
pub mod semantic;

pub use semantic::SemanticColors;

use crate::Theme;

impl Theme {
    /// Get semantic colors for this theme
    pub fn colors(&self) -> SemanticColors {
        match self {
            Theme::Light => semantic::light(),
            Theme::Dark => semantic::dark(),
            Theme::Sepia => semantic::sepia(),
            Theme::Ocean => semantic::ocean(),
            Theme::Twilight => semantic::twilight(),
        }
    }
}
