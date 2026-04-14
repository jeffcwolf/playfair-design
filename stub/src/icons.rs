//! Lucide icon helpers.
//!
//! Each public function returns an [`iced::widget::Text`] configured with the
//! Lucide icon font ([`super::LUCIDE_FONT`]).  The text content is the Unicode
//! codepoint for the corresponding icon.

use iced::widget::Text;

// ---------------------------------------------------------------------------
// Navigation & general
// ---------------------------------------------------------------------------

/// Left arrow icon.
pub fn arrow_left<'a>() -> Text<'a> {
    iced::widget::text("\u{f300}").font(super::LUCIDE_FONT)
}

/// Close / X icon.
pub fn x<'a>() -> Text<'a> {
    iced::widget::text("\u{f236}").font(super::LUCIDE_FONT)
}

/// Checkmark icon.
pub fn check<'a>() -> Text<'a> {
    iced::widget::text("\u{f3e9}").font(super::LUCIDE_FONT)
}

/// Search (magnifying glass) icon.
pub fn search<'a>() -> Text<'a> {
    iced::widget::text("\u{f194}").font(super::LUCIDE_FONT)
}

/// Settings (gear) icon.
pub fn settings<'a>() -> Text<'a> {
    iced::widget::text("\u{f007}").font(super::LUCIDE_FONT)
}

/// Refresh icon.
pub fn refresh<'a>() -> Text<'a> {
    iced::widget::text("\u{f3d7}").font(super::LUCIDE_FONT)
}

/// Eye (visible) icon.
pub fn eye<'a>() -> Text<'a> {
    iced::widget::text("\u{f33a}").font(super::LUCIDE_FONT)
}

/// Eye-off (hidden) icon.
pub fn eye_off<'a>() -> Text<'a> {
    iced::widget::text("\u{f35c}").font(super::LUCIDE_FONT)
}

/// Plus icon.
pub fn plus<'a>() -> Text<'a> {
    iced::widget::text("\u{f482}").font(super::LUCIDE_FONT)
}

// ---------------------------------------------------------------------------
// File & folder operations
// ---------------------------------------------------------------------------

/// File icon.
pub fn file<'a>() -> Text<'a> {
    iced::widget::text("\u{f17f}").font(super::LUCIDE_FONT)
}

/// File-plus icon.
pub fn file_plus<'a>() -> Text<'a> {
    iced::widget::text("\u{f427}").font(super::LUCIDE_FONT)
}

/// Folder-open icon.
pub fn folder_open<'a>() -> Text<'a> {
    iced::widget::text("\u{f24a}").font(super::LUCIDE_FONT)
}

/// Folder-plus icon.
pub fn folder_plus<'a>() -> Text<'a> {
    iced::widget::text("\u{f1e4}").font(super::LUCIDE_FONT)
}

/// Save (floppy disk) icon.
pub fn save<'a>() -> Text<'a> {
    iced::widget::text("\u{f508}").font(super::LUCIDE_FONT)
}

/// Upload icon.
pub fn upload<'a>() -> Text<'a> {
    iced::widget::text("\u{f085}").font(super::LUCIDE_FONT)
}

/// Download icon.
pub fn download<'a>() -> Text<'a> {
    iced::widget::text("\u{f0fd}").font(super::LUCIDE_FONT)
}

/// Trash icon.
pub fn trash<'a>() -> Text<'a> {
    iced::widget::text("\u{f5ce}").font(super::LUCIDE_FONT)
}

/// Pencil icon.
pub fn pencil<'a>() -> Text<'a> {
    iced::widget::text("\u{f263}").font(super::LUCIDE_FONT)
}

/// Pencil-line icon.
pub fn pencil_line<'a>() -> Text<'a> {
    iced::widget::text("\u{f2c9}").font(super::LUCIDE_FONT)
}

// ---------------------------------------------------------------------------
// App-specific
// ---------------------------------------------------------------------------

/// Quote icon.
pub fn quote<'a>() -> Text<'a> {
    iced::widget::text("\u{f1da}").font(super::LUCIDE_FONT)
}

/// Notebook icon.
pub fn notebook<'a>() -> Text<'a> {
    iced::widget::text("\u{f41e}").font(super::LUCIDE_FONT)
}

/// Book-open icon.
pub fn book_open<'a>() -> Text<'a> {
    iced::widget::text("\u{f01d}").font(super::LUCIDE_FONT)
}

/// Newspaper icon.
pub fn newspaper<'a>() -> Text<'a> {
    iced::widget::text("\u{f4ea}").font(super::LUCIDE_FONT)
}

/// Panel-left icon.
pub fn panel_left<'a>() -> Text<'a> {
    iced::widget::text("\u{f3de}").font(super::LUCIDE_FONT)
}

/// Panel-right icon.
pub fn panel_right<'a>() -> Text<'a> {
    iced::widget::text("\u{f571}").font(super::LUCIDE_FONT)
}

/// Columns-two (split view) icon.
pub fn columns_two<'a>() -> Text<'a> {
    iced::widget::text("\u{f55c}").font(super::LUCIDE_FONT)
}

// ---------------------------------------------------------------------------
// Navigation
// ---------------------------------------------------------------------------

/// Chevron-left icon.
pub fn chevron_left<'a>() -> Text<'a> {
    iced::widget::text("\u{f251}").font(super::LUCIDE_FONT)
}

/// Chevron-right icon.
pub fn chevron_right<'a>() -> Text<'a> {
    iced::widget::text("\u{f22f}").font(super::LUCIDE_FONT)
}

/// Chevron-down icon.
pub fn chevron_down<'a>() -> Text<'a> {
    iced::widget::text("\u{f2b7}").font(super::LUCIDE_FONT)
}

/// Chevron-up icon.
pub fn chevron_up<'a>() -> Text<'a> {
    iced::widget::text("\u{f20d}").font(super::LUCIDE_FONT)
}

/// Arrow-right icon.
pub fn arrow_right<'a>() -> Text<'a> {
    iced::widget::text("\u{f278}").font(super::LUCIDE_FONT)
}

/// External-link icon.
pub fn external_link<'a>() -> Text<'a> {
    iced::widget::text("\u{f3a0}").font(super::LUCIDE_FONT)
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

/// Copy icon.
pub fn copy<'a>() -> Text<'a> {
    iced::widget::text("\u{f24e}").font(super::LUCIDE_FONT)
}

/// Undo-2 icon.
pub fn undo_2<'a>() -> Text<'a> {
    iced::widget::text("\u{f1d2}").font(super::LUCIDE_FONT)
}

/// Filter (funnel) icon.
pub fn filter<'a>() -> Text<'a> {
    iced::widget::text("\u{f59b}").font(super::LUCIDE_FONT)
}

/// More-horizontal (ellipsis) icon.
pub fn more_horizontal<'a>() -> Text<'a> {
    iced::widget::text("\u{f4d2}").font(super::LUCIDE_FONT)
}

// ---------------------------------------------------------------------------
// File & document
// ---------------------------------------------------------------------------

/// File-text icon.
pub fn file_text<'a>() -> Text<'a> {
    iced::widget::text("\u{f2b1}").font(super::LUCIDE_FONT)
}

/// File-check icon.
pub fn file_check<'a>() -> Text<'a> {
    iced::widget::text("\u{f099}").font(super::LUCIDE_FONT)
}

/// File-x icon.
pub fn file_x<'a>() -> Text<'a> {
    iced::widget::text("\u{f1a1}").font(super::LUCIDE_FONT)
}

/// Paperclip icon.
pub fn paperclip<'a>() -> Text<'a> {
    iced::widget::text("\u{f43f}").font(super::LUCIDE_FONT)
}

/// Link icon.
pub fn link<'a>() -> Text<'a> {
    iced::widget::text("\u{f17a}").font(super::LUCIDE_FONT)
}

// ---------------------------------------------------------------------------
// Data / charts
// ---------------------------------------------------------------------------

/// Bar-chart (chart-bar) icon.
pub fn bar_chart<'a>() -> Text<'a> {
    iced::widget::text("\u{f691}").font(super::LUCIDE_FONT)
}

/// Bar-chart-2 (chart-bar-big) icon.
pub fn bar_chart_2<'a>() -> Text<'a> {
    iced::widget::text("\u{f07e}").font(super::LUCIDE_FONT)
}

/// Line-chart (chart-line) icon.
pub fn line_chart<'a>() -> Text<'a> {
    iced::widget::text("\u{f581}").font(super::LUCIDE_FONT)
}

/// Pie-chart (chart-pie) icon.
pub fn pie_chart<'a>() -> Text<'a> {
    iced::widget::text("\u{f493}").font(super::LUCIDE_FONT)
}

/// Trending-up icon.
pub fn trending_up<'a>() -> Text<'a> {
    iced::widget::text("\u{f49c}").font(super::LUCIDE_FONT)
}

/// Trending-down icon.
pub fn trending_down<'a>() -> Text<'a> {
    iced::widget::text("\u{f4e0}").font(super::LUCIDE_FONT)
}

// ---------------------------------------------------------------------------
// Status / feedback
// ---------------------------------------------------------------------------

/// Check-circle (circle-check) icon.
pub fn check_circle<'a>() -> Text<'a> {
    iced::widget::text("\u{f580}").font(super::LUCIDE_FONT)
}

/// X-circle (circle-x) icon.
pub fn x_circle<'a>() -> Text<'a> {
    iced::widget::text("\u{f100}").font(super::LUCIDE_FONT)
}

/// Alert-circle (circle-alert) icon.
pub fn alert_circle<'a>() -> Text<'a> {
    iced::widget::text("\u{f03b}").font(super::LUCIDE_FONT)
}

/// Alert-triangle (triangle-alert) icon.
pub fn alert_triangle<'a>() -> Text<'a> {
    iced::widget::text("\u{f47a}").font(super::LUCIDE_FONT)
}

/// Info icon.
pub fn info<'a>() -> Text<'a> {
    iced::widget::text("\u{f225}").font(super::LUCIDE_FONT)
}

/// Help-circle (circle-question-mark) icon.
pub fn help_circle<'a>() -> Text<'a> {
    iced::widget::text("\u{f20c}").font(super::LUCIDE_FONT)
}

/// Clock icon.
pub fn clock<'a>() -> Text<'a> {
    iced::widget::text("\u{f2f9}").font(super::LUCIDE_FONT)
}

/// Loader icon.
pub fn loader<'a>() -> Text<'a> {
    iced::widget::text("\u{f4cb}").font(super::LUCIDE_FONT)
}

// ---------------------------------------------------------------------------
// UI / layout
// ---------------------------------------------------------------------------

/// Bookmark icon.
pub fn bookmark<'a>() -> Text<'a> {
    iced::widget::text("\u{f4d9}").font(super::LUCIDE_FONT)
}

/// Bookmark-plus icon.
pub fn bookmark_plus<'a>() -> Text<'a> {
    iced::widget::text("\u{f51d}").font(super::LUCIDE_FONT)
}

/// Tag icon.
pub fn tag<'a>() -> Text<'a> {
    iced::widget::text("\u{f10b}").font(super::LUCIDE_FONT)
}

/// Calendar icon.
pub fn calendar<'a>() -> Text<'a> {
    iced::widget::text("\u{f3ea}").font(super::LUCIDE_FONT)
}

/// Hash icon.
pub fn hash<'a>() -> Text<'a> {
    iced::widget::text("\u{f2ae}").font(super::LUCIDE_FONT)
}

/// Minus icon.
pub fn minus<'a>() -> Text<'a> {
    iced::widget::text("\u{f573}").font(super::LUCIDE_FONT)
}

/// Grip-vertical icon.
pub fn grip_vertical<'a>() -> Text<'a> {
    iced::widget::text("\u{f578}").font(super::LUCIDE_FONT)
}

/// Maximize-2 icon.
pub fn maximize_2<'a>() -> Text<'a> {
    iced::widget::text("\u{f596}").font(super::LUCIDE_FONT)
}

/// Minimize-2 icon.
pub fn minimize_2<'a>() -> Text<'a> {
    iced::widget::text("\u{f5b7}").font(super::LUCIDE_FONT)
}

/// Zoom-in icon.
pub fn zoom_in<'a>() -> Text<'a> {
    iced::widget::text("\u{f1ae}").font(super::LUCIDE_FONT)
}

/// Zoom-out icon.
pub fn zoom_out<'a>() -> Text<'a> {
    iced::widget::text("\u{f18c}").font(super::LUCIDE_FONT)
}

/// List icon.
pub fn list<'a>() -> Text<'a> {
    iced::widget::text("\u{f531}").font(super::LUCIDE_FONT)
}

/// Table icon.
pub fn table<'a>() -> Text<'a> {
    iced::widget::text("\u{f190}").font(super::LUCIDE_FONT)
}

// ---------------------------------------------------------------------------
// Writing / editor
// ---------------------------------------------------------------------------

/// Bold icon.
pub fn bold<'a>() -> Text<'a> {
    iced::widget::text("\u{f2ba}").font(super::LUCIDE_FONT)
}

/// Italic icon.
pub fn italic<'a>() -> Text<'a> {
    iced::widget::text("\u{f1bf}").font(super::LUCIDE_FONT)
}

/// Type icon (`type` is a Rust keyword, so named `type_icon`).
pub fn type_icon<'a>() -> Text<'a> {
    iced::widget::text("\u{f25a}").font(super::LUCIDE_FONT)
}
