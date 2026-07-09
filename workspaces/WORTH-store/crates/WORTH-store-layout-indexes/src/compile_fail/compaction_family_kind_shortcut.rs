//! ```compile_fail
//! use worth_store_contracts::LayoutCompactionFamilyKind;
//! use worth_store_layout_indexes::layout_declarations;
//!
//! let kind = LayoutCompactionFamilyKind::LayoutCompactionUnit;
//! let _ = layout_declarations().admit_existing_family(&kind);
//! ```
