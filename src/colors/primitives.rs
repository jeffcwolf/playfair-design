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
