//! Compatibility re-exports for callers that still import from `worth_ui::facade`.
//!
//! Prefer the named facade submodules such as `app`, `inspection`, `diagnostics`,
//! `dsl`, `host`, `registry`, and `support` for new code.

pub use worth_ui_runtime::facade::*;
