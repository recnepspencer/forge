//! Forge UI component library.
//!
//! DOMAIN: All visual widgets used by the Forge UI. Every component is a
//! pure function over egui::Ui + ForgeTheme + props. No state, no side effects.
//! DEPENDENCIES: forge-ui-theme, forge-ui-types, egui.

pub mod button;
pub mod badge;
pub mod alert;
pub mod icons;

pub use button::{fg_button, FgButton, FgButtonSize, FgButtonVariant};
pub use badge::{fg_badge, FgBadge, FgBadgeVariant};
pub use alert::{fg_alert, FgAlert, FgAlertVariant};
pub use icons::{FgIcon, IconStore};
