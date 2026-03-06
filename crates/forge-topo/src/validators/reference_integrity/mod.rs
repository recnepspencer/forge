//! Reference integrity and ownership validators.
//!
//! DOMAIN: Pointer/ownership/orphan checks — ensuring every referenced
//! handle exists, every entity has exactly one owner, and no entities
//! are unreachable from body roots.
//!
//! STRUCTURE:
//!   hierarchy.rs            — Parent-child containment hierarchy
//!   dangling_refs.rs        — Dangling half-edge references
//!   bidirectional_links.rs  — Representative pointer reciprocity
//!   face_loop_existence.rs  — Face outer_loop validity
//!   single_owner.rs         — Single owner per loop
//!   inner_outer_consistency.rs — Inner/outer loop domain consistency
//!   generational_freshness.rs  — Stale generational ID detection

mod acyclic_containment;
mod bidirectional_links;
mod dangling_refs;
mod face_loop_existence;
mod generational_freshness;
mod hierarchy;
mod inner_outer_consistency;
mod orphan_half_edges;
mod single_owner;

use forge_core::KernelError;

pub(crate) use acyclic_containment::validate_acyclic_containment;
pub(crate) use bidirectional_links::validate_bidirectional_links;
pub(crate) use dangling_refs::validate_no_dangling_half_edge_refs;
pub(crate) use face_loop_existence::validate_face_has_at_least_one_loop;
pub(crate) use generational_freshness::validate_generational_id_freshness;
pub(crate) use hierarchy::validate_hierarchy;
pub(crate) use inner_outer_consistency::validate_inner_outer_loop_consistency;
pub(crate) use orphan_half_edges::validate_no_orphan_half_edges;
pub(crate) use single_owner::validate_single_owner_per_loop;

pub(crate) use super::shared::vf;
