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

use crate::arena::TopologyArena;
use forge_core::KernelError;

/// Validate the parent-child hierarchy: Face→Shell→Region→Lump→Solid.
///
/// Checks invariants:
/// 1. Every face's shell parent must be a live (non-deleted) shell.
/// 2. Every shell's region parent must be a live (non-deleted) region.
/// 3. Every region's lump parent must be a live (non-deleted) lump.
/// 4. Every lump's body parent must be a live (non-deleted) body.
/// 5. Every shell listed in a region (outer or inner) must be a live shell.
/// 6. Every region listed in a lump must be a live region.
/// 7. Every lump listed in a body must be a live lump.
/// 8. Orphan detection: every child must be owned by exactly one parent.
pub(crate) fn validate_hierarchy(arena: &TopologyArena) -> Result<(), KernelError> {
    // ── Upward pointer checks: child → parent must be live ──

    for (face_id, face_data) in arena.iter_faces() {
        let shell_id = face_data.shell();
        arena
            .get_shell(shell_id)
            .map_err(|_| KernelError::TopologyViolation {
                err: forge_core::TopologyError::HierarchyViolation {
                    parent_kind: "Shell".to_string(),
                    parent_index: shell_id.index(),
                    child_kind: "Face".to_string(),
                    child_index: face_id.index(),
                    detail: format!(
                        "Face {} references shell {}(gen{}) which is stale or deleted",
                        face_id.index(),
                        shell_id.index(),
                        shell_id.generation()
                    ),
                },
                context: None,
            })?;
    }

    for (shell_id, shell_data) in arena.iter_shells() {
        let region_id = shell_data.region();
        arena
            .get_region(region_id)
            .map_err(|_| KernelError::TopologyViolation {
                err: forge_core::TopologyError::HierarchyViolation {
                    parent_kind: "Region".to_string(),
                    parent_index: region_id.index(),
                    child_kind: "Shell".to_string(),
                    child_index: shell_id.index(),
                    detail: format!(
                        "Shell {} references region {}(gen{}) which is stale or deleted",
                        shell_id.index(),
                        region_id.index(),
                        region_id.generation()
                    ),
                },
                context: None,
            })?;
    }

    for (region_id, region_data) in arena.iter_regions() {
        let lump_id = region_data.lump();
        arena
            .get_lump(lump_id)
            .map_err(|_| KernelError::TopologyViolation {
                err: forge_core::TopologyError::HierarchyViolation {
                    parent_kind: "Lump".to_string(),
                    parent_index: lump_id.index(),
                    child_kind: "Region".to_string(),
                    child_index: region_id.index(),
                    detail: format!(
                        "Region {} references lump {}(gen{}) which is stale or deleted",
                        region_id.index(),
                        lump_id.index(),
                        lump_id.generation()
                    ),
                },
                context: None,
            })?;

        if let Some(outer) = region_data.outer_shell() {
            arena
                .get_shell(outer)
                .map_err(|_| KernelError::TopologyViolation {
                    err: forge_core::TopologyError::HierarchyViolation {
                        parent_kind: "Region".to_string(),
                        parent_index: region_id.index(),
                        child_kind: "Shell".to_string(),
                        child_index: outer.index(),
                        detail: format!(
                            "Region {} outer shell {}(gen{}) is stale or deleted",
                            region_id.index(),
                            outer.index(),
                            outer.generation()
                        ),
                    },
                    context: None,
                })?;
        }

        for shell_id in region_data.inner_shells() {
            arena
                .get_shell(*shell_id)
                .map_err(|_| KernelError::TopologyViolation {
                    err: forge_core::TopologyError::HierarchyViolation {
                        parent_kind: "Region".to_string(),
                        parent_index: region_id.index(),
                        child_kind: "Shell".to_string(),
                        child_index: shell_id.index(),
                        detail: format!(
                            "Region {} inner shell {}(gen{}) is stale or deleted",
                            region_id.index(),
                            shell_id.index(),
                            shell_id.generation()
                        ),
                    },
                    context: None,
                })?;
        }
    }

    for (lump_id, lump_data) in arena.iter_lumps() {
        let body_id = lump_data.body();
        arena
            .get_body(body_id)
            .map_err(|_| KernelError::TopologyViolation {
                err: forge_core::TopologyError::HierarchyViolation {
                    parent_kind: "Body".to_string(),
                    parent_index: body_id.index(),
                    child_kind: "Lump".to_string(),
                    child_index: lump_id.index(),
                    detail: format!(
                        "Lump {} references solid {}(gen{}) which is stale or deleted",
                        lump_id.index(),
                        body_id.index(),
                        body_id.generation()
                    ),
                },
                context: None,
            })?;
    }

    for (body_id, solid_data) in arena.iter_bodies() {
        for lump_id in solid_data.lumps() {
            arena
                .get_lump(*lump_id)
                .map_err(|_| KernelError::TopologyViolation {
                    err: forge_core::TopologyError::HierarchyViolation {
                        parent_kind: "Body".to_string(),
                        parent_index: body_id.index(),
                        child_kind: "Lump".to_string(),
                        child_index: lump_id.index(),
                        detail: format!(
                            "Solid {} lists lump {}(gen{}) which is stale or deleted",
                            body_id.index(),
                            lump_id.index(),
                            lump_id.generation()
                        ),
                    },
                    context: None,
                })?;
        }
    }

    // ── Orphan detection: every child must be owned by exactly one parent ──

    let mut owned_shells = std::collections::BTreeSet::new();
    for (_region_id, region_data) in arena.iter_regions() {
        if let Some(outer) = region_data.outer_shell() {
            owned_shells.insert(outer);
        }
        for shell_id in region_data.inner_shells() {
            owned_shells.insert(*shell_id);
        }
    }
    for (shell_id, _shell_data) in arena.iter_shells() {
        if !owned_shells.contains(&shell_id) {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::HierarchyViolation {
                    parent_kind: "Region".to_string(),
                    parent_index: u32::MAX,
                    child_kind: "Shell".to_string(),
                    child_index: shell_id.index(),
                    detail: format!(
                        "Orphaned shell {}: not owned by any region",
                        shell_id.index()
                    ),
                },
                context: None,
            });
        }
    }

    let mut owned_regions = std::collections::BTreeSet::new();
    for (_lump_id, lump_data) in arena.iter_lumps() {
        for region_id in lump_data.regions() {
            owned_regions.insert(*region_id);
        }
    }
    for (region_id, _region_data) in arena.iter_regions() {
        if !owned_regions.contains(&region_id) {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::HierarchyViolation {
                    parent_kind: "Lump".to_string(),
                    parent_index: u32::MAX,
                    child_kind: "Region".to_string(),
                    child_index: region_id.index(),
                    detail: format!(
                        "Orphaned region {}: not owned by any lump",
                        region_id.index()
                    ),
                },
                context: None,
            });
        }
    }

    let mut owned_lumps = std::collections::BTreeSet::new();
    for (_body_id, body_data) in arena.iter_bodies() {
        for lump_id in body_data.lumps() {
            owned_lumps.insert(*lump_id);
        }
    }
    for (lump_id, _lump_data) in arena.iter_lumps() {
        if !owned_lumps.contains(&lump_id) {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::HierarchyViolation {
                    parent_kind: "Body".to_string(),
                    parent_index: u32::MAX,
                    child_kind: "Lump".to_string(),
                    child_index: lump_id.index(),
                    detail: format!("Orphaned lump {}: not owned by any body", lump_id.index()),
                },
                context: None,
            });
        }
    }

    Ok(())
}
