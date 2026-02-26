//! Forge UI component library.
//!
//! DOMAIN: All visual widgets used by the Forge UI. Every component is a
//! pure function over egui::Ui + ForgeTheme + props. No state, no side effects.
//! DEPENDENCIES: forge-ui-theme, forge-ui-types, egui.

// ── Atoms ────────────────────────────────────────────────────────────────────
pub mod button;
pub mod badge;
pub mod alert;
pub mod toast;
pub mod icons;
pub mod card;
pub mod chip;
pub mod icon_button;
pub mod page_tab;
pub mod input;
pub mod textarea;
pub mod modal;
pub mod dropdown;
pub mod form;

// ── Molecules ────────────────────────────────────────────────────────────────
pub mod search_bar;
pub mod feature_row;

// ── Re-exports ───────────────────────────────────────────────────────────────
pub use button::{fg_button, FgButton, FgButtonSize, FgButtonVariant};
pub use badge::{fg_badge, FgBadge, FgBadgeVariant};
pub use alert::{fg_alert, FgAlert, AlertAction};
pub use toast::{fg_toast, FgToast, FgToastVariant};
pub use icons::{FgIcon, IconStore};
pub use card::{fg_card, FgCard, FgCardVariant};
pub use chip::{fg_chip, FgChip};
pub use icon_button::{fg_icon_button, FgIconButton};
pub use page_tab::{fg_page_tab, FgPageTab};
pub use search_bar::{fg_search_bar, SearchBarProps};
pub use feature_row::{fg_feature_row, FeatureRowProps};
pub use input::{fg_input, FgInput};
pub use textarea::{fg_textarea, FgTextArea};
pub use modal::fg_modal;
pub use dropdown::{fg_dropdown, FgDropdown, DropdownItem, DropdownState};
pub use form::fg_form;
