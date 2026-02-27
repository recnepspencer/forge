//! Parametric Boolean Pipeline — split → classify → select → assemble → postprocess.
//!
//! DOMAIN: General-purpose Boolean pipeline that handles all geometry types
//! (planar, NURBS, analytic surfaces). Uses face splitting, ray-cast
//! classification, and halfedge stitching.
//!
//! DEPENDENCIES: `split/`, `assemble/`, `postprocess/`,
//!               `traits` (engine abstraction), `engines/` (concrete engines)
//!               classify is in `shared_steps::classify_faces`
//!
//! INVARIANTS:
//! - All topology decisions use `CertifiedTriSign` (D3)
//! - Operations are atomic via `MutableDraft` (D6)
//! - Result satisfies Euler's formula (V - E + F = 2)

pub mod assemble;
pub mod engines;
pub(crate) mod postprocess;
pub(crate) mod split;
pub mod traits;
