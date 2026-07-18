//! egui-backed presentational widgets using [`worth_ui_theme::WorthTheme`].
//!
//! Widget functions render into an `egui::Ui` and may participate in egui's
//! response and memory model. They do not own Worth declaration or runtime
//! semantics.

// ── Atoms ────────────────────────────────────────────────────────────────────
pub mod alert;
pub mod badge;
pub mod button;
pub mod card;
pub mod chip;
pub mod dropdown;
pub mod form;
pub mod icon_button;
pub mod icons;
pub mod input;
pub mod modal;
pub mod page_tab;
pub mod textarea;
pub mod toast;

// ── Molecules ────────────────────────────────────────────────────────────────
pub mod feature_row;
pub mod search_bar;

// ── Re-exports ───────────────────────────────────────────────────────────────
pub use alert::{fg_alert, AlertAction, FgAlert};
pub use badge::{fg_badge, FgBadge, FgBadgeVariant};
pub use button::{fg_button, FgButton, FgButtonSize, FgButtonVariant};
pub use card::{fg_card, FgCard, FgCardVariant};
pub use chip::{fg_chip, FgChip};
pub use dropdown::{fg_dropdown, DropdownItem, DropdownState, FgDropdown};
pub use feature_row::{fg_feature_row, FeatureRowProps};
pub use form::fg_form;
pub use icon_button::{fg_icon_button, FgIconButton};
pub use icons::{FgIcon, IconStore};
pub use input::{fg_input, FgInput};
pub use modal::fg_modal;
pub use page_tab::{fg_page_tab, FgPageTab};
pub use search_bar::{fg_search_bar, SearchBarProps};
pub use textarea::{fg_textarea, FgTextArea};
pub use toast::{fg_toast, FgToast, FgToastVariant};
