//! Regression seeder scaffold.
//!
//! DOMAIN: Every confirmed bug becomes a permanent, named test scenario.
//! Each function documents the original issue and produces the exact
//! geometry configuration that triggered the bug.
//!
//! As the kernel matures, this file grows. It never shrinks —
//! regressions are permanent guardians.

// ── Placeholder ──────────────────────────────────────────────────────────────
//
// No confirmed regressions yet. When the first one is found, add a
// seeder function like:
//
// ```rust
// /// Regression #42: near-coplanar faces at 1e-11 offset cause
// /// spurious edge splits in the boolean classifier.
// ///
// /// See: https://github.com/org/forge/issues/42
// pub fn regression_42_near_coplanar() -> Result<(SolidEnvelope, SolidEnvelope), KernelError> {
//     use crate::integration_tests::harness::shapes;
//     let a = shapes::cube([0.0, 0.0, 0.0], 1.0)?;
//     let b = shapes::cube([0.0, 0.0, 1e-11], 1.0)?;
//     Ok((a, b))
// }
// ```
