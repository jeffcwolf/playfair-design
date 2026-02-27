//! Styles for iced_aw extended widgets.
//!
//! Enable with `features = ["iced_aw"]` in your Cargo.toml.
//!
//! These modules follow the same pattern as the native widget styles:
//! each public function takes a `&Theme` and returns a closure suitable
//! for passing to the widget's `.style()` method.
//!
//! # Note
//!
//! This module targets `iced_aw` 0.13 which is compatible with `iced` 0.14.
//! Some widgets listed in earlier drafts (e.g. `modal`, `spinner`) do not
//! have a `Catalog`/`Style` system in this version and are therefore omitted.

pub mod badge;
pub mod card;
pub mod number_input;
pub mod tab_bar;
