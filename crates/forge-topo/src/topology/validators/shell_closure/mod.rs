//! Shell and body closure/orientation validators.
//!
//! DOMAIN: Watertightness for solid shells, laminar-only boundaries
//! for sheet bodies, consistent shell orientation (outward for outer,
//! inward for inner), inner shell containment, and self-intersection
//! detection at the topology level.
//!
//! VALIDATORS (from validators.md §5):
//! - ValidateShellWatertightness
//! - ValidateBoundaryIsLaminarOnly
//! - ValidateConsistentShellOrientation
//! - ValidateInnerShellContainment
//! - ValidateNoInsideOutShells
//! - ValidateNoSelfIntersectingShellTopology
//!
//! DEPENDENCIES: `arena`, `handles`, `queries::shell`
