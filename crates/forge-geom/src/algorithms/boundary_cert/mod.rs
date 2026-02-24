//! Boundary certification for merge eligibility.
//!
//! DOMAIN: Certify that a face-group boundary is geometrically valid for merge
//! (simple or weakly simple). Two-phase algorithm: fast-path crossing check
//! via exact orient2d predicates, fallback to arrangement-based recognizer.
//!
//! - `schema`: Data shapes (`ProjectionFrame2D`, `Segment2D`, `WeakSimpleCertificate`, ...)
//! - `eval`: Two-phase certifier algorithm
//! - `tests`: Spec §9.2 test matrix
//!
//! DEPENDENCIES: `forge-math` (Shewchuk orient2d exact predicates)
//! INVARIANTS: Stateless, pure. No topology, no policy, no thresholds.

pub mod schema;
pub mod eval;

#[cfg(test)]
mod tests;
