// playfair-design/src/elevation.rs
//! Elevation system using shadows to create visual depth
//!
//! Inspired by Material Design and macOS, this creates a sense of layering
//! in the UI by lifting elements above the background.

use iced::{Color, Shadow, Vector};

/// Shadow elevation levels
pub mod shadow {
    use super::*;

    /// No shadow (flat)
    pub fn none() -> Shadow {
        Shadow {
            color: Color::TRANSPARENT,
            offset: Vector::ZERO,
            blur_radius: 0.0,
        }
    }

    /// Level 1: Subtle lift (cards, buttons)
    /// Used for: Cards, elevated panels, primary buttons
    pub fn sm() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 3.0,
        }
    }

    /// Level 2: Medium elevation (modals, dropdowns)
    /// Used for: Dropdown menus, tooltips, floating panels
    pub fn md() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        }
    }

    /// Level 3: High elevation (overlays, popovers)
    /// Used for: Modals, dialogs, important overlays
    pub fn lg() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.16),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        }
    }

    /// Level 4: Floating (top-level menus, tooltips)
    /// Used for: Context menus, floating action buttons
    pub fn xl() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.20),
            offset: Vector::new(0.0, 12.0),
            blur_radius: 32.0,
        }
    }
}
