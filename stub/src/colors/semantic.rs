// playfair-design/src/colors/semantic.rs
//! Semantic color tokens that map to specific UI purposes
//!
//! These colors have meaningful names that describe their purpose,
//! not their appearance. This allows themes to change while maintaining
//! semantic consistency.

use super::primitives::{dark, ember, light, ocean, sage, sepia, slate, twilight};
use iced::Color;

/// Semantic colors for UI components
#[derive(Debug, Clone, Copy)]
pub struct SemanticColors {
    // Surface colors (backgrounds, cards, panels)
    pub surface_primary: Color,
    pub surface_secondary: Color,
    pub surface_tertiary: Color,
    pub surface_hover: Color,
    pub surface_selected: Color,
    pub surface_disabled: Color,

    // Text colors
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_tertiary: Color,
    pub text_disabled: Color,
    pub text_on_primary: Color,

    // Border colors
    pub border_default: Color,
    pub border_hover: Color,
    pub border_focus: Color,
    pub border_divider: Color,

    // Action colors (interactive elements)
    pub action_primary: Color,
    pub action_primary_hover: Color,
    pub action_primary_active: Color,
    pub action_secondary: Color,
    pub action_secondary_hover: Color,
    pub action_danger: Color,
    pub action_danger_hover: Color,
    pub action_success: Color,
    pub action_warning: Color,

    // State colors (feedback)
    pub state_info: Color,
    pub state_success: Color,
    pub state_warning: Color,
    pub state_error: Color,
}

/// Light theme semantic colors
pub fn light() -> SemanticColors {
    SemanticColors {
        // Surfaces
        surface_primary: light::GRAY_50,
        surface_secondary: Color::WHITE,
        surface_tertiary: light::GRAY_100,
        surface_hover: light::GRAY_200,
        surface_selected: light::BLUE_50,
        surface_disabled: light::GRAY_200,

        // Text
        text_primary: light::GRAY_900,
        text_secondary: light::GRAY_600,
        text_tertiary: light::GRAY_500,
        text_disabled: light::GRAY_400,
        text_on_primary: Color::WHITE,

        // Borders
        border_default: light::GRAY_300,
        border_hover: light::GRAY_400,
        border_focus: light::BLUE_500,
        border_divider: light::GRAY_200,

        // Actions
        action_primary: light::BLUE_500,
        action_primary_hover: light::BLUE_600,
        action_primary_active: light::BLUE_700,
        action_secondary: light::GRAY_100,
        action_secondary_hover: light::GRAY_200,
        action_danger: light::RED_500,
        action_danger_hover: light::RED_600,
        action_success: light::GREEN_500,
        action_warning: light::ORANGE_500,

        // States
        state_info: light::BLUE_500,
        state_success: light::GREEN_500,
        state_warning: light::ORANGE_500,
        state_error: light::RED_500,
    }
}

/// Dark theme semantic colors
pub fn dark() -> SemanticColors {
    SemanticColors {
        // Surfaces
        surface_primary: dark::GRAY_50,
        surface_secondary: dark::GRAY_100,
        surface_tertiary: dark::GRAY_200,
        surface_hover: dark::GRAY_300,
        surface_selected: Color::from_rgba(0.06, 0.52, 1.0, 0.2), // Blue with alpha
        surface_disabled: dark::GRAY_300,

        // Text
        text_primary: dark::GRAY_900,
        text_secondary: dark::GRAY_600,
        text_tertiary: dark::GRAY_500,
        text_disabled: dark::GRAY_400,
        text_on_primary: Color::WHITE,

        // Borders
        border_default: dark::GRAY_300,
        border_hover: dark::GRAY_400,
        border_focus: dark::BLUE_500,
        border_divider: dark::GRAY_200,

        // Actions
        action_primary: dark::BLUE_500,
        action_primary_hover: dark::BLUE_400,
        action_primary_active: dark::BLUE_600,
        action_secondary: dark::GRAY_200,
        action_secondary_hover: dark::GRAY_300,
        action_danger: dark::RED_500,
        action_danger_hover: dark::RED_600,
        action_success: dark::GREEN_500,
        action_warning: dark::ORANGE_500,

        // States
        state_info: dark::BLUE_500,
        state_success: dark::GREEN_500,
        state_warning: dark::ORANGE_500,
        state_error: dark::RED_500,
    }
}

/// Ocean theme semantic colors — cool blue-gray light theme
pub fn ocean() -> SemanticColors {
    SemanticColors {
        // Surfaces
        surface_primary: ocean::SURFACE,
        surface_secondary: ocean::SURFACE_2,
        surface_tertiary: ocean::SURFACE_3,
        surface_hover: ocean::GRAY_200,
        surface_selected: Color::from_rgba(0.22, 0.47, 0.84, 0.12), // BLUE_500 at ~12% alpha
        surface_disabled: ocean::GRAY_200,

        // Text
        text_primary: ocean::GRAY_900,
        text_secondary: ocean::GRAY_600,
        text_tertiary: ocean::GRAY_500,
        text_disabled: ocean::GRAY_400,
        text_on_primary: Color::WHITE,

        // Borders
        border_default: ocean::GRAY_300,
        border_hover: ocean::GRAY_400,
        border_focus: ocean::BLUE_500,
        border_divider: ocean::GRAY_200,

        // Actions
        action_primary: ocean::BLUE_500,
        action_primary_hover: ocean::BLUE_600,
        action_primary_active: ocean::BLUE_600,
        action_secondary: ocean::GRAY_100,
        action_secondary_hover: ocean::GRAY_200,
        action_danger: ocean::RED_500,
        action_danger_hover: Color::from_rgb(0.90, 0.20, 0.17),
        action_success: ocean::GREEN_500,
        action_warning: ocean::ORANGE_500,

        // States
        state_info: ocean::BLUE_500,
        state_success: ocean::GREEN_500,
        state_warning: ocean::ORANGE_500,
        state_error: ocean::RED_500,
    }
}

/// Twilight theme semantic colors — deep purple-black dark theme
pub fn twilight() -> SemanticColors {
    SemanticColors {
        // Surfaces
        surface_primary: twilight::GRAY_50,
        surface_secondary: twilight::GRAY_100,
        surface_tertiary: twilight::GRAY_200,
        surface_hover: twilight::GRAY_300,
        surface_selected: Color::from_rgba(0.60, 0.38, 0.97, 0.25), // VIOLET_500 at ~25% alpha
        surface_disabled: twilight::GRAY_300,

        // Text
        text_primary: twilight::GRAY_900,
        text_secondary: twilight::GRAY_600,
        text_tertiary: twilight::GRAY_500,
        text_disabled: twilight::GRAY_400,
        text_on_primary: Color::WHITE,

        // Borders
        border_default: twilight::GRAY_300,
        border_hover: twilight::GRAY_400,
        border_focus: twilight::VIOLET_500,
        border_divider: twilight::GRAY_200,

        // Actions
        action_primary: twilight::VIOLET_500,
        action_primary_hover: twilight::VIOLET_400,
        action_primary_active: twilight::VIOLET_600,
        action_secondary: twilight::GRAY_200,
        action_secondary_hover: twilight::GRAY_300,
        action_danger: twilight::RED_500,
        action_danger_hover: Color::from_rgb(0.90, 0.36, 0.34),
        action_success: twilight::GREEN_500,
        action_warning: twilight::ORANGE_500,

        // States
        state_info: twilight::VIOLET_500,
        state_success: twilight::GREEN_500,
        state_warning: twilight::ORANGE_500,
        state_error: twilight::RED_500,
    }
}

/// Sepia theme semantic colors
pub fn sepia() -> SemanticColors {
    SemanticColors {
        // Surfaces
        surface_primary: sepia::WARM_100,
        surface_secondary: sepia::WARM_50,
        surface_tertiary: sepia::WARM_200,
        surface_hover: sepia::WARM_300,
        surface_selected: Color::from_rgb(0.62, 0.50, 0.43), // Brown tint
        surface_disabled: sepia::WARM_300,

        // Text
        text_primary: sepia::WARM_900,
        text_secondary: sepia::WARM_600,
        text_tertiary: sepia::WARM_500,
        text_disabled: sepia::WARM_400,
        text_on_primary: Color::WHITE,

        // Borders
        border_default: sepia::WARM_300,
        border_hover: sepia::WARM_400,
        border_focus: sepia::BROWN_500,
        border_divider: sepia::WARM_200,

        // Actions
        action_primary: sepia::BROWN_500,
        action_primary_hover: sepia::BROWN_600,
        action_primary_active: sepia::BROWN_600,
        action_secondary: sepia::WARM_200,
        action_secondary_hover: sepia::WARM_300,
        action_danger: sepia::RED_500,
        action_danger_hover: Color::from_rgb(0.70, 0.31, 0.31),
        action_success: sepia::GREEN_500,
        action_warning: sepia::ORANGE_500,

        // States
        state_info: sepia::BROWN_500,
        state_success: sepia::GREEN_500,
        state_warning: sepia::ORANGE_500,
        state_error: sepia::RED_500,
    }
}

/// Sage theme semantic colors — muted green-gray light-mid theme
pub fn sage() -> SemanticColors {
    SemanticColors {
        // Surfaces
        surface_primary: sage::SURFACE,
        surface_secondary: sage::SURFACE_2,
        surface_tertiary: sage::SURFACE_3,
        surface_hover: sage::GRAY_300,
        surface_selected: Color::from_rgba(0.28, 0.53, 0.42, 0.15), // GREEN_500 at ~15% alpha
        surface_disabled: sage::GRAY_300,

        // Text
        text_primary: sage::GRAY_900,
        text_secondary: sage::GRAY_600,
        text_tertiary: sage::GRAY_500,
        text_disabled: sage::GRAY_400,
        text_on_primary: Color::WHITE,

        // Borders
        border_default: sage::GRAY_300,
        border_hover: sage::GRAY_400,
        border_focus: sage::GREEN_500,
        border_divider: sage::GRAY_200,

        // Actions
        action_primary: sage::GREEN_500,
        action_primary_hover: sage::GREEN_600,
        action_primary_active: sage::GREEN_600,
        action_secondary: sage::GRAY_200,
        action_secondary_hover: sage::GRAY_300,
        action_danger: sage::STATUS_RED,
        action_danger_hover: Color::from_rgb(0.68, 0.30, 0.28), // #AD4D47
        action_success: sage::STATUS_GREEN,
        action_warning: sage::STATUS_ORANGE,

        // States
        state_info: sage::GREEN_500,
        state_success: sage::STATUS_GREEN,
        state_warning: sage::STATUS_ORANGE,
        state_error: sage::STATUS_RED,
    }
}

/// Slate theme semantic colors — cool blue-gray dark-mid theme
pub fn slate() -> SemanticColors {
    SemanticColors {
        // Surfaces
        surface_primary: slate::SURFACE,
        surface_secondary: slate::SURFACE_2,
        surface_tertiary: slate::SURFACE_3,
        surface_hover: slate::GRAY_300,
        surface_selected: Color::from_rgba(0.40, 0.60, 0.82, 0.20), // STEEL_500 at ~20% alpha
        surface_disabled: slate::GRAY_300,

        // Text
        text_primary: slate::GRAY_900,
        text_secondary: slate::GRAY_600,
        text_tertiary: slate::GRAY_500,
        text_disabled: slate::GRAY_400,
        text_on_primary: Color::WHITE,

        // Borders
        border_default: slate::GRAY_300,
        border_hover: slate::GRAY_400,
        border_focus: slate::STEEL_500,
        border_divider: slate::GRAY_200,

        // Actions
        action_primary: slate::STEEL_500,
        action_primary_hover: slate::STEEL_400,
        action_primary_active: slate::STEEL_600,
        action_secondary: slate::GRAY_200,
        action_secondary_hover: slate::GRAY_300,
        action_danger: slate::STATUS_RED,
        action_danger_hover: Color::from_rgb(0.90, 0.34, 0.32), // #E65752
        action_success: slate::STATUS_GREEN,
        action_warning: slate::STATUS_ORANGE,

        // States
        state_info: slate::STEEL_500,
        state_success: slate::STATUS_GREEN,
        state_warning: slate::STATUS_ORANGE,
        state_error: slate::STATUS_RED,
    }
}

/// Ember theme semantic colors — warm dark-mid theme with copper accents
pub fn ember() -> SemanticColors {
    SemanticColors {
        // Surfaces
        surface_primary: ember::SURFACE,
        surface_secondary: ember::SURFACE_2,
        surface_tertiary: ember::SURFACE_3,
        surface_hover: ember::GRAY_300,
        surface_selected: Color::from_rgba(0.80, 0.52, 0.30, 0.20), // COPPER_500 at ~20% alpha
        surface_disabled: ember::GRAY_300,

        // Text
        text_primary: ember::GRAY_900,
        text_secondary: ember::GRAY_600,
        text_tertiary: ember::GRAY_500,
        text_disabled: ember::GRAY_400,
        text_on_primary: Color::WHITE,

        // Borders
        border_default: ember::GRAY_300,
        border_hover: ember::GRAY_400,
        border_focus: ember::COPPER_500,
        border_divider: ember::GRAY_200,

        // Actions
        action_primary: ember::COPPER_500,
        action_primary_hover: ember::COPPER_400,
        action_primary_active: ember::COPPER_600,
        action_secondary: ember::GRAY_200,
        action_secondary_hover: ember::GRAY_300,
        action_danger: ember::STATUS_RED,
        action_danger_hover: Color::from_rgb(0.90, 0.38, 0.31), // #E6614F
        action_success: ember::STATUS_GREEN,
        action_warning: ember::STATUS_ORANGE,

        // States
        state_info: ember::COPPER_500,
        state_success: ember::STATUS_GREEN,
        state_warning: ember::STATUS_ORANGE,
        state_error: ember::STATUS_RED,
    }
}
