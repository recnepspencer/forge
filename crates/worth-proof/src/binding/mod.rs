//! Bindings and per-axis drift.
//!
//! A *binding* is the set of facts a capability was issued against. Presenting
//! it later asks one question — has anything drifted? — and the answer worth
//! having is not `false` but **which axis**.
//!
//! Milestone 9.16 Phase 8 hand-wrote a thirteen-field binding, eleven
//! comparisons, and eleven denial kinds. Every comparison was correct; the
//! audit's question was whether any axis had been *forgotten*, and nothing in
//! the code could answer it. [`crate::binding_axes!`] generates the comparison
//! and the denial enum from one declaration, and
//! [`crate::binding_axis_drift_certification!`] fails the build when a
//! declared axis has no drift test.

mod authoring;
mod axes;

pub use axes::{Binding, BindingAxes};
