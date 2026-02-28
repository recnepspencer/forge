//! Reference integrity and ownership validators.
//!
//! DOMAIN: Pointer/ownership/orphan checks — ensuring every referenced
//! handle exists, every entity has exactly one owner, and no entities
//! are unreachable from body roots.
//!
//! VALIDATORS (from validators.md §1):
//! - ValidateNoDanglingHandles
//! - ValidateGenerationalIdMatchesStorage
//! - ValidateSingleOwnerPerEntity
//! - ValidateNoDoubleOwnedEntities
//! - ValidateNoOrphanEntities
//! - ValidateBidirectionalLinks
//! - ValidateAcyclicContainmentGraph
//!
//! DEPENDENCIES: `arena` (entity storage), `handles` (typed IDs)
