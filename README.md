# Playfair Design

The shared design system for the [Playfair Suite](https://github.com/YOUR_USERNAME/playfair) — an Integrated Scholarship Environment for humanities researchers.

Playfair Design provides semantic colour palettes, spacing and border tokens, and ready-made style functions for [Iced](https://iced.rs) 0.14 widgets. Every app in the suite depends on this crate so that buttons, panels, text, and inputs look and feel consistent whether the scholar is managing sources in Playfair, writing in Scribe, or analysing manuscripts in Quire.

## What It Provides

**Themes.** Eight colour themes, selectable at runtime via a theme picker available in every app:

| Theme | Character |
|-------|-----------|
| Light | Neutral greys, clean white surfaces |
| Dark | Low-light environment, reduced eye strain |
| Sepia | Warm aged-paper tones (default) |
| Ocean | Cool blue-green |
| Twilight | Muted purple-blue evening |
| Sage | Soft green-grey, natural |
| Slate | Cool grey, professional |
| Ember | Warm amber-red |

Each theme defines a complete `SemanticColors` set: 24 colour tokens organised into text, surface, action, and border categories. App code never references raw colours — it uses semantic names like `text_primary`, `surface_secondary`, `action_danger`, `border_focus`.

**Design tokens.** Spacing constants (`spacing::SM`, `MD`, `LG`, `XL`) and border tokens (`borders::radius::SM/MD/LG`, `borders::width::HAIRLINE/DEFAULT/THICK`) for consistent layout across all apps.

**Style functions.** Pre-built Iced widget style closures that map semantic colours to widget states (active, hovered, pressed, disabled, focused):

- `styles::btn` — `primary`, `secondary`, `danger`, `success`, `ghost`, `minimal`
- `styles::cont` — `card`, `panel`, `sidebar`, `section`, `info`, `warning`, `success`, `inline`
- `styles::txt` — `h1`, `h2`, `h3`, `h4`, `body`, `caption`, `label`
- `styles::text_input` — styled text inputs with focus states
- `styles::pl` — styled pick lists
- `styles::prog` — styled progress bars
- `styles::scroll` — styled scrollable containers

Every style function takes a `&Theme` and returns a closure compatible with Iced's widget styling API.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
playfair-design = { path = "../playfair-design" }
```

In your app:

```rust
use playfair_design::{Theme, ThemeState, styles};

// Store in your app state
let theme_state = ThemeState::new(Theme::Sepia);

// Use in views
let save_button = button("Save")
    .style(styles::btn::primary(&theme_state.theme));

let panel = container(content)
    .style(styles::cont::card(&theme_state.theme));

let heading = text("Sources")
    .style(styles::txt::h2(&theme_state.theme));
```

The `ThemeState` struct provides `to_iced()` for the Iced application builder and `colors()` for direct access to semantic colour values when building custom widgets.

## Building

Requires Rust 1.75+ and Iced 0.14.

```bash
cargo build
cargo test
```

## Part of the Playfair Suite

Playfair Design is a library crate, not a standalone application. It is used by:

- **Playfair** — the command centre (collection management, universal search, research dashboard)
- **Scribe** — integrated academic writing environment
- **Quire** — palaeographic manuscript analysis workbench
- **Almanak** — OCR transcription and annotation tool
- **Proso** — named entity manager and biographical research tool

## Licence

Part of the Playfair Suite. See the repository root for licence terms.