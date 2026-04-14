// playfair-design/src/typography/scale.rs
//! Typography scale with harmonious font sizes
//!
//! Based on a modular scale (1.250 - Major Third) for visual rhythm

/// Font size tokens in pixels
pub mod size {
    /// Display text (48px) - Hero sections, splash screens
    pub const DISPLAY: f32 = 48.0;

    /// H1 headings (34px) - Page titles
    pub const H1: f32 = 34.0;

    /// H2 headings (28px) - Section titles
    pub const H2: f32 = 28.0;

    /// H3 headings (22px) - Subsection titles
    pub const H3: f32 = 22.0;

    /// H4 headings (17px) - Group titles
    pub const H4: f32 = 17.0;

    /// H5 headings (13px) - Small headings, tab labels
    pub const H5: f32 = 13.0;

    /// H6 headings (11px) - Tiny headings
    pub const H6: f32 = 11.0;

    /// Body large (15px) - Emphasized body text
    pub const BODY_LG: f32 = 15.0;

    /// Body default (13px) - Standard body text
    pub const BODY: f32 = 13.0;

    /// Body small (12px) - De-emphasized body text
    pub const BODY_SM: f32 = 12.0;

    /// Caption (11px) - Labels, captions
    pub const CAPTION: f32 = 11.0;

    /// Tiny (10px) - Footnotes, metadata
    pub const TINY: f32 = 10.0;

    /// Code (12px) - Monospace code text
    pub const CODE: f32 = 12.0;
}

/// Line height multipliers for optimal readability
pub mod line_height {
    /// Tight (1.2) - Headings, titles
    pub const TIGHT: f32 = 1.2;

    /// Normal (1.5) - Body text
    pub const NORMAL: f32 = 1.5;

    /// Relaxed (1.7) - Long-form reading
    pub const RELAXED: f32 = 1.7;

    /// Code (1.6) - Monospace code blocks
    pub const CODE: f32 = 1.6;
}

/// Letter spacing adjustments (in em units)
pub mod letter_spacing {
    /// Tight (-0.02em) - Large headings
    pub const TIGHT: f32 = -0.02;

    /// Normal (0em) - Body text
    pub const NORMAL: f32 = 0.0;

    /// Wide (0.05em) - Small caps, labels
    pub const WIDE: f32 = 0.05;
}
