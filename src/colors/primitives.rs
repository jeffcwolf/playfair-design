// playfair-core/src/colors/primitives.rs
//! Raw color primitives organized by theme and hue
//!
//! These are the base colors that semantic tokens are built from.

use iced::Color;

/// Light theme color primitives
pub mod light {
    use super::Color;

    // Neutrals (grays) - 50 to 900 scale
    pub const GRAY_50: Color = Color::from_rgb(0.98, 0.98, 0.98); // #FAFAFA
    pub const GRAY_100: Color = Color::from_rgb(0.96, 0.96, 0.96); // #F5F5F5
    pub const GRAY_200: Color = Color::from_rgb(0.93, 0.93, 0.93); // #EEEEEE
    pub const GRAY_300: Color = Color::from_rgb(0.88, 0.88, 0.88); // #E0E0E0
    pub const GRAY_400: Color = Color::from_rgb(0.74, 0.74, 0.74); // #BDBDBD
    pub const GRAY_500: Color = Color::from_rgb(0.62, 0.62, 0.62); // #9E9E9E
    pub const GRAY_600: Color = Color::from_rgb(0.46, 0.46, 0.46); // #757575
    pub const GRAY_700: Color = Color::from_rgb(0.38, 0.38, 0.38); // #616161
    pub const GRAY_800: Color = Color::from_rgb(0.26, 0.26, 0.26); // #424242
    pub const GRAY_900: Color = Color::from_rgb(0.13, 0.13, 0.13); // #212121

    // Primary (blue) - macOS system blue
    pub const BLUE_50: Color = Color::from_rgb(0.94, 0.97, 1.0); // #EFF7FF
    pub const BLUE_100: Color = Color::from_rgb(0.87, 0.93, 1.0); // #DDF0FF
    pub const BLUE_200: Color = Color::from_rgb(0.73, 0.86, 0.98); // #BBDCFA
    pub const BLUE_300: Color = Color::from_rgb(0.58, 0.77, 0.96); // #94C4F5
    pub const BLUE_400: Color = Color::from_rgb(0.40, 0.66, 0.95); // #66A9F2
    pub const BLUE_500: Color = Color::from_rgb(0.0, 0.48, 0.94); // #007AFF (macOS blue)
    pub const BLUE_600: Color = Color::from_rgb(0.0, 0.41, 0.82); // #0068D1
    pub const BLUE_700: Color = Color::from_rgb(0.0, 0.34, 0.71); // #0056B5
    pub const BLUE_800: Color = Color::from_rgb(0.0, 0.28, 0.59); // #004896
    pub const BLUE_900: Color = Color::from_rgb(0.0, 0.22, 0.47); // #003878

    // Success (green) - macOS system green
    pub const GREEN_500: Color = Color::from_rgb(0.20, 0.78, 0.35); // #34C759
    pub const GREEN_600: Color = Color::from_rgb(0.17, 0.68, 0.30); // #2BAD4D
    pub const GREEN_700: Color = Color::from_rgb(0.14, 0.58, 0.26); // #249341

    // Warning (orange) - macOS system orange
    pub const ORANGE_500: Color = Color::from_rgb(1.0, 0.58, 0.0); // #FF9500
    pub const ORANGE_600: Color = Color::from_rgb(0.90, 0.52, 0.0); // #E68400
    pub const ORANGE_700: Color = Color::from_rgb(0.80, 0.46, 0.0); // #CC7400

    // Danger (red) - macOS system red
    pub const RED_500: Color = Color::from_rgb(1.0, 0.23, 0.19); // #FF3B30
    pub const RED_600: Color = Color::from_rgb(0.90, 0.20, 0.17); // #E6332A
    pub const RED_700: Color = Color::from_rgb(0.80, 0.18, 0.15); // #CC2D25
}

/// Dark theme color primitives
pub mod dark {
    use super::Color;

    // Neutrals (inverted - darker numbers are lighter)
    pub const GRAY_50: Color = Color::from_rgb(0.13, 0.13, 0.13); // #212121
    pub const GRAY_100: Color = Color::from_rgb(0.16, 0.16, 0.16); // #292929
    pub const GRAY_200: Color = Color::from_rgb(0.19, 0.19, 0.19); // #303030
    pub const GRAY_300: Color = Color::from_rgb(0.24, 0.24, 0.24); // #3D3D3D
    pub const GRAY_400: Color = Color::from_rgb(0.38, 0.38, 0.38); // #616161
    pub const GRAY_500: Color = Color::from_rgb(0.54, 0.54, 0.54); // #8A8A8A
    pub const GRAY_600: Color = Color::from_rgb(0.67, 0.67, 0.67); // #ABABAB
    pub const GRAY_700: Color = Color::from_rgb(0.78, 0.78, 0.78); // #C7C7C7
    pub const GRAY_800: Color = Color::from_rgb(0.87, 0.87, 0.87); // #DEDEDE
    pub const GRAY_900: Color = Color::from_rgb(0.95, 0.95, 0.95); // #F2F2F2

    // Primary (blue) - slightly brighter for dark theme
    pub const BLUE_400: Color = Color::from_rgb(0.40, 0.66, 0.95); // #66A9F2
    pub const BLUE_500: Color = Color::from_rgb(0.06, 0.52, 1.0); // #0F85FF
    pub const BLUE_600: Color = Color::from_rgb(0.0, 0.48, 0.94); // #007AFF

    // Success (green)
    pub const GREEN_500: Color = Color::from_rgb(0.20, 0.84, 0.35); // #34D759
    pub const GREEN_600: Color = Color::from_rgb(0.20, 0.78, 0.35); // #34C759

    // Warning (orange)
    pub const ORANGE_500: Color = Color::from_rgb(1.0, 0.62, 0.04); // #FF9E0A
    pub const ORANGE_600: Color = Color::from_rgb(1.0, 0.58, 0.0); // #FF9500

    // Danger (red)
    pub const RED_500: Color = Color::from_rgb(1.0, 0.27, 0.23); // #FF453A
    pub const RED_600: Color = Color::from_rgb(1.0, 0.23, 0.19); // #FF3B30
}

/// Ocean theme color primitives — cool blue-gray surfaces with steel blue accents
pub mod ocean {
    use super::Color;

    pub const GRAY_50: Color = Color::from_rgb(0.96, 0.97, 0.99); // #F5F8FC — main surface
    pub const GRAY_100: Color = Color::from_rgb(0.91, 0.94, 0.98); // #E8EFFA
    pub const GRAY_200: Color = Color::from_rgb(0.86, 0.90, 0.96); // #DBE5F5
    pub const GRAY_300: Color = Color::from_rgb(0.77, 0.83, 0.93); // #C4D4EE
    pub const GRAY_400: Color = Color::from_rgb(0.60, 0.69, 0.82); // #99B0D1
    pub const GRAY_500: Color = Color::from_rgb(0.44, 0.54, 0.68); // #718AAD
    pub const GRAY_600: Color = Color::from_rgb(0.30, 0.40, 0.55); // #4D668C
    pub const GRAY_700: Color = Color::from_rgb(0.22, 0.31, 0.45); // #384F73
    pub const GRAY_800: Color = Color::from_rgb(0.14, 0.22, 0.34); // #243857
    pub const GRAY_900: Color = Color::from_rgb(0.09, 0.15, 0.25); // #172640

    pub const BLUE_400: Color = Color::from_rgb(0.38, 0.60, 0.92); // #6199EB
    pub const BLUE_500: Color = Color::from_rgb(0.22, 0.47, 0.84); // #3878D6
    pub const BLUE_600: Color = Color::from_rgb(0.16, 0.38, 0.72); // #2960B8

    pub const SURFACE: Color = Color::from_rgb(0.96, 0.97, 0.99); // #F5F8FC
    pub const SURFACE_2: Color = Color::from_rgb(1.0, 1.0, 1.0); // white card bg
    pub const SURFACE_3: Color = Color::from_rgb(0.91, 0.94, 0.98); // #E8EFFA

    pub const GREEN_500: Color = Color::from_rgb(0.20, 0.78, 0.35);
    pub const ORANGE_500: Color = Color::from_rgb(1.0, 0.58, 0.0);
    pub const RED_500: Color = Color::from_rgb(1.0, 0.23, 0.19);
}

/// Twilight theme color primitives — deep purple-black surfaces with violet accents
pub mod twilight {
    use super::Color;

    pub const GRAY_50: Color = Color::from_rgb(0.09, 0.07, 0.14); // #171226 — deep base
    pub const GRAY_100: Color = Color::from_rgb(0.12, 0.10, 0.18); // #1E1A2E
    pub const GRAY_200: Color = Color::from_rgb(0.16, 0.13, 0.24); // #29213D
    pub const GRAY_300: Color = Color::from_rgb(0.22, 0.18, 0.32); // #382E52
    pub const GRAY_400: Color = Color::from_rgb(0.38, 0.32, 0.52); // #615285
    pub const GRAY_500: Color = Color::from_rgb(0.55, 0.48, 0.70); // #8C7AB3
    pub const GRAY_600: Color = Color::from_rgb(0.68, 0.62, 0.82); // #AD9ED1
    pub const GRAY_700: Color = Color::from_rgb(0.78, 0.74, 0.90); // #C7BCE6
    pub const GRAY_800: Color = Color::from_rgb(0.88, 0.85, 0.96); // #E0D9F5
    pub const GRAY_900: Color = Color::from_rgb(0.94, 0.92, 0.99); // #F0EBFC

    pub const VIOLET_400: Color = Color::from_rgb(0.72, 0.52, 1.0); // #B885FF
    pub const VIOLET_500: Color = Color::from_rgb(0.60, 0.38, 0.97); // #9961F7
    pub const VIOLET_600: Color = Color::from_rgb(0.50, 0.28, 0.87); // #8047DE

    pub const GREEN_500: Color = Color::from_rgb(0.24, 0.84, 0.44);
    pub const ORANGE_500: Color = Color::from_rgb(1.0, 0.72, 0.22);
    pub const RED_500: Color = Color::from_rgb(1.0, 0.40, 0.38);
}

/// Sepia theme color primitives
pub mod sepia {
    use super::Color;

    // Warm neutrals
    pub const WARM_50: Color = Color::from_rgb(0.98, 0.96, 0.94); // #FAF5F0
    pub const WARM_100: Color = Color::from_rgb(0.96, 0.94, 0.91); // #F5F0E8
    pub const WARM_200: Color = Color::from_rgb(0.93, 0.90, 0.86); // #EEE5DB
    pub const WARM_300: Color = Color::from_rgb(0.87, 0.84, 0.79); // #DED6C9
    pub const WARM_400: Color = Color::from_rgb(0.74, 0.70, 0.64); // #BDB3A3
    pub const WARM_500: Color = Color::from_rgb(0.62, 0.58, 0.52); // #9E9485
    pub const WARM_600: Color = Color::from_rgb(0.46, 0.43, 0.39); // #756D63
    pub const WARM_700: Color = Color::from_rgb(0.38, 0.34, 0.31); // #61584F
    pub const WARM_800: Color = Color::from_rgb(0.26, 0.23, 0.21); // #423B35
    pub const WARM_900: Color = Color::from_rgb(0.15, 0.13, 0.12); // #26211E

    // Warm brown primary
    pub const BROWN_400: Color = Color::from_rgb(0.62, 0.50, 0.43); // #9E806D
    pub const BROWN_500: Color = Color::from_rgb(0.55, 0.43, 0.39); // #8C6D63
    pub const BROWN_600: Color = Color::from_rgb(0.48, 0.37, 0.33); // #7A5E54

    // Muted green
    pub const GREEN_500: Color = Color::from_rgb(0.55, 0.63, 0.39); // #8CA063

    // Muted orange
    pub const ORANGE_500: Color = Color::from_rgb(0.85, 0.65, 0.30); // #D9A64D

    // Muted red
    pub const RED_500: Color = Color::from_rgb(0.78, 0.35, 0.35); // #C75959
}

/// Sage theme color primitives — soft green-gray surfaces with forest-teal accents
pub mod sage {
    use super::Color;

    // Green-gray neutrals — 50 to 900 scale
    pub const GRAY_50: Color = Color::from_rgb(0.92, 0.94, 0.90); // #EBF0E6
    pub const GRAY_100: Color = Color::from_rgb(0.89, 0.91, 0.87); // #E3E8DE
    pub const GRAY_200: Color = Color::from_rgb(0.85, 0.87, 0.83); // #D9DED4
    pub const GRAY_300: Color = Color::from_rgb(0.78, 0.80, 0.76); // #C7CCC2
    pub const GRAY_400: Color = Color::from_rgb(0.64, 0.67, 0.62); // #A3AB9E
    pub const GRAY_500: Color = Color::from_rgb(0.50, 0.54, 0.48); // #808A7A
    pub const GRAY_600: Color = Color::from_rgb(0.36, 0.40, 0.34); // #5C6657
    pub const GRAY_700: Color = Color::from_rgb(0.28, 0.32, 0.26); // #475242
    pub const GRAY_800: Color = Color::from_rgb(0.20, 0.24, 0.19); // #333D30
    pub const GRAY_900: Color = Color::from_rgb(0.16, 0.19, 0.15); // #293026

    // Forest/teal green accent
    pub const GREEN_400: Color = Color::from_rgb(0.34, 0.60, 0.49); // #57997D
    pub const GREEN_500: Color = Color::from_rgb(0.28, 0.53, 0.42); // #47876B
    pub const GREEN_600: Color = Color::from_rgb(0.22, 0.45, 0.35); // #387359

    pub const SURFACE: Color = Color::from_rgb(0.82, 0.84, 0.80); // #D1D6CC
    pub const SURFACE_2: Color = Color::from_rgb(0.85, 0.87, 0.83); // #D9DED4
    pub const SURFACE_3: Color = Color::from_rgb(0.78, 0.80, 0.76); // #C7CCC2

    // Muted status colors (similar approach to Sepia)
    pub const STATUS_GREEN: Color = Color::from_rgb(0.40, 0.62, 0.38); // #669E61
    pub const STATUS_ORANGE: Color = Color::from_rgb(0.80, 0.62, 0.28); // #CC9E47
    pub const STATUS_RED: Color = Color::from_rgb(0.75, 0.34, 0.32); // #BF5752
}

/// Slate theme color primitives — cool blue-gray dark surfaces with steel blue accents
pub mod slate {
    use super::Color;

    // Cool blue-gray neutrals — lower numbers = darker, higher = lighter (dark theme convention)
    pub const GRAY_50: Color = Color::from_rgb(0.14, 0.16, 0.20); // #242933
    pub const GRAY_100: Color = Color::from_rgb(0.18, 0.20, 0.24); // #2E333D
    pub const GRAY_200: Color = Color::from_rgb(0.22, 0.24, 0.28); // #383D47
    pub const GRAY_300: Color = Color::from_rgb(0.28, 0.31, 0.36); // #474F5C
    pub const GRAY_400: Color = Color::from_rgb(0.40, 0.44, 0.50); // #667080
    pub const GRAY_500: Color = Color::from_rgb(0.52, 0.56, 0.63); // #858FA1
    pub const GRAY_600: Color = Color::from_rgb(0.62, 0.66, 0.72); // #9EA8B8
    pub const GRAY_700: Color = Color::from_rgb(0.74, 0.77, 0.82); // #BDC4D1
    pub const GRAY_800: Color = Color::from_rgb(0.84, 0.86, 0.90); // #D6DBE6
    pub const GRAY_900: Color = Color::from_rgb(0.90, 0.92, 0.95); // #E6EBF2

    // Steel blue accent
    pub const STEEL_400: Color = Color::from_rgb(0.50, 0.68, 0.90); // #80ADE6
    pub const STEEL_500: Color = Color::from_rgb(0.40, 0.60, 0.82); // #6699D1
    pub const STEEL_600: Color = Color::from_rgb(0.32, 0.52, 0.73); // #5285BA

    pub const SURFACE: Color = Color::from_rgb(0.22, 0.24, 0.28); // #383D47
    pub const SURFACE_2: Color = Color::from_rgb(0.26, 0.28, 0.32); // #424752
    pub const SURFACE_3: Color = Color::from_rgb(0.18, 0.20, 0.24); // #2E333D

    // Bright status colors for dark background
    pub const STATUS_GREEN: Color = Color::from_rgb(0.30, 0.82, 0.45); // #4DD173
    pub const STATUS_ORANGE: Color = Color::from_rgb(1.0, 0.68, 0.18); // #FFAD2E
    pub const STATUS_RED: Color = Color::from_rgb(1.0, 0.38, 0.36); // #FF615C
}

/// Ember theme color primitives — warm dark charcoal surfaces with copper/burnt-orange accents
pub mod ember {
    use super::Color;

    // Warm brown-gray neutrals — lower numbers = darker, higher = lighter (dark theme convention)
    pub const GRAY_50: Color = Color::from_rgb(0.11, 0.09, 0.08); // #1C1714
    pub const GRAY_100: Color = Color::from_rgb(0.14, 0.12, 0.11); // #241E1C
    pub const GRAY_200: Color = Color::from_rgb(0.18, 0.15, 0.14); // #2E2624
    pub const GRAY_300: Color = Color::from_rgb(0.25, 0.21, 0.19); // #403630
    pub const GRAY_400: Color = Color::from_rgb(0.40, 0.35, 0.31); // #66594F
    pub const GRAY_500: Color = Color::from_rgb(0.54, 0.48, 0.43); // #8A7A6E
    pub const GRAY_600: Color = Color::from_rgb(0.64, 0.58, 0.52); // #A39485
    pub const GRAY_700: Color = Color::from_rgb(0.76, 0.70, 0.64); // #C2B3A3
    pub const GRAY_800: Color = Color::from_rgb(0.85, 0.80, 0.75); // #D9CCBF
    pub const GRAY_900: Color = Color::from_rgb(0.92, 0.89, 0.85); // #EBE3D9

    // Copper/burnt-orange accent
    pub const COPPER_400: Color = Color::from_rgb(0.88, 0.60, 0.36); // #E0995C
    pub const COPPER_500: Color = Color::from_rgb(0.80, 0.52, 0.30); // #CC854D
    pub const COPPER_600: Color = Color::from_rgb(0.70, 0.44, 0.24); // #B3703D

    pub const SURFACE: Color = Color::from_rgb(0.18, 0.15, 0.14); // #2E2624
    pub const SURFACE_2: Color = Color::from_rgb(0.22, 0.19, 0.17); // #38302B
    pub const SURFACE_3: Color = Color::from_rgb(0.14, 0.12, 0.11); // #241E1C

    // Warm-tinted bright status colors for dark background
    pub const STATUS_GREEN: Color = Color::from_rgb(0.35, 0.78, 0.40); // #59C766
    pub const STATUS_ORANGE: Color = Color::from_rgb(1.0, 0.70, 0.25); // #FFB340
    pub const STATUS_RED: Color = Color::from_rgb(1.0, 0.42, 0.35); // #FF6B59
}
