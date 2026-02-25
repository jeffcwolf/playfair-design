// playfair-core/src/borders.rs
//! Border radius and width tokens for consistent styling

/// Border radius values
pub mod radius {
    /// No rounding (sharp corners)
    pub const NONE: f32 = 0.0;

    /// Extra-small rounding (3px)
    /// Used for: Controls — small buttons, inputs
    pub const XS: f32 = 3.0;

    /// Small rounding (5px)
    /// Used for: Standard controls, buttons, chips
    pub const SM: f32 = 5.0;

    /// Medium rounding (8px)
    /// Used for: Cards, panels
    pub const MD: f32 = 8.0;

    /// Large rounding (12px)
    /// Used for: Modals, large cards, important containers
    pub const LG: f32 = 12.0;

    /// Extra large rounding (16px)
    /// Used for: Hero elements, special components
    pub const XL: f32 = 16.0;

    /// Pill shape (999px)
    /// Used for: Tags, badges, pill buttons
    pub const PILL: f32 = 999.0;
}

/// Border width values
pub mod width {
    /// Hairline border (1px)
    /// Used for: Subtle dividers, very light borders
    pub const HAIRLINE: f32 = 1.0;

    /// Standard border (1.5px)
    /// Used for: Default borders, better on Retina displays
    pub const DEFAULT: f32 = 1.5;

    /// Thick border (2px)
    /// Used for: Emphasis, focus states
    pub const THICK: f32 = 2.0;

    /// Extra thick border (3px)
    /// Used for: Strong emphasis, error states
    pub const EXTRA_THICK: f32 = 3.0;
}
