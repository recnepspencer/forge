//! Parametric Boolean Pipeline — split → classify → select → assemble → postprocess.
//!
//! DOMAIN: General-purpose Boolean pipeline that handles all geometry types
//! (planar, NURBS, analytic surfaces). Uses face splitting, ray-cast
//! classification, and halfedge stitching.
//!
//! DEPENDENCIES: `split/`, `classify/`, `assemble/`, `postprocess/`,
//!               `traits` (engine abstraction), `engines/` (concrete engines)
//!
//! INVARIANTS:
//! - All topology decisions use `CertifiedTriSign` (D3)
//! - Operations are atomic via `MutableDraft` (D6)
//! - Result satisfies Euler's formula (V - E + F = 2)

pub(crate) mod split;
pub(crate) mod classify;
pub(crate) mod postprocess;
pub mod assemble;
pub mod traits;
pub mod engines;
