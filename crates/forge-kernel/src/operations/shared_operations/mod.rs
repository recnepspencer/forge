//! Shared kernel operations — cross-cutting atomic operations consumed by all feature domains.
//!
//! DOMAIN: Foundational kernel writes (placement, stitching, snapping) that
//!         primitives, booleans, Euler operators, and NURBS tessellation all depend on.
//!
//! RULE: Nothing in this module is feature-specific. If a domain needs it,
//!       it calls through here — it does not duplicate the logic.

pub mod facade;
pub(crate) mod placement;
