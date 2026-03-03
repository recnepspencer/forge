//! Shared kernel validators — cross-cutting invariant checks for traced operations.
//!
//! DOMAIN: Decision log validation for operations consumed across all feature domains.
//! Validators here check that the recorded decisions match expected patterns
//! (correct kind, threshold, entity scope, margin).
//!
//! RULE: Validators return `Result<(), KernelError>`. Tests call `.unwrap()`.
//!       Production code can handle errors gracefully.

pub mod facade;
pub(crate) mod input;
pub(crate) mod placement;
