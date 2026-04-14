// playfair-design/src/typography/weights.rs
//! Font weight tokens for typography hierarchy

use iced::font::Weight;

/// Light weight (300) - Decorative, large text
pub const LIGHT: Weight = Weight::Light;

/// Regular weight (400) - Body text
pub const REGULAR: Weight = Weight::Normal;

/// Medium weight (500) - Emphasized text
pub const MEDIUM: Weight = Weight::Medium;

/// Semibold weight (600) - Headings
pub const SEMIBOLD: Weight = Weight::Semibold;

/// Bold weight (700) - Strong emphasis
pub const BOLD: Weight = Weight::Bold;
