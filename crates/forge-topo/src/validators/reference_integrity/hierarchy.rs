//! Parent-child containment hierarchy validator.
//!
//! INVARIANT: Every shell→region, region→lump, lump→body pointer must
//! refer to a live entity that claims the child. No orphaned shells.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

/// Validate parent-child hierarchy: every shell→region, region→lump,
/// lump→body pointer must refer to a live entity that actually claims
/// the child in its child list.
pub(crate) fn validate_hierarchy(arena: &TopologyArena) -> Result<(), KernelError> {
    // Shells → Regions
    for (shell_id, shell_data) in arena.iter_shells() {
        let parent = shell_data.region();
        let region_data =
            arena
                .get_region(parent)
                .map_err(|_| KernelError::TopologyViolation {
                    err: forge_core::TopologyError::HierarchyViolation {
                        parent_kind: "Region".to_string(),
                        parent_index: parent.index(),
                        child_kind: "Shell".to_string(),
                        child_index: shell_id.index(),
                        detail: "Shell's parent region is stale/deleted".to_string(),
                    },
                    context: None,
                })?;
        if !region_data.shells().contains(&shell_id) {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::HierarchyViolation {
                    parent_kind: "Region".to_string(),
                    parent_index: parent.index(),
                    child_kind: "Shell".to_string(),
                    child_index: shell_id.index(),
                    detail: "Shell's parent region does not list it as a child".to_string(),
                },
                context: None,
            });
        }
    }

    // Regions → Lumps
    for (region_id, region_data) in arena.iter_regions() {
        let parent = region_data.lump();
        let lump_data =
            arena
                .get_lump(parent)
                .map_err(|_| KernelError::TopologyViolation {
                    err: forge_core::TopologyError::HierarchyViolation {
                        parent_kind: "Lump".to_string(),
                        parent_index: parent.index(),
                        child_kind: "Region".to_string(),
                        child_index: region_id.index(),
                        detail: "Region's parent lump is stale/deleted".to_string(),
                    },
                    context: None,
                })?;
        if !lump_data.regions().contains(&region_id) {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::HierarchyViolation {
                    parent_kind: "Lump".to_string(),
                    parent_index: parent.index(),
                    child_kind: "Region".to_string(),
                    child_index: region_id.index(),
                    detail: "Region's parent lump does not list it as a child".to_string(),
                },
                context: None,
            });
        }
    }

    // Lumps → Bodies
    for (lump_id, lump_data) in arena.iter_lumps() {
        let parent = lump_data.body();
        let body_data =
            arena
                .get_body(parent)
                .map_err(|_| KernelError::TopologyViolation {
                    err: forge_core::TopologyError::HierarchyViolation {
                        parent_kind: "Body".to_string(),
                        parent_index: parent.index(),
                        child_kind: "Lump".to_string(),
                        child_index: lump_id.index(),
                        detail: "Lump's parent body is stale/deleted".to_string(),
                    },
                    context: None,
                })?;
        if !body_data.lumps().contains(&lump_id) {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::HierarchyViolation {
                    parent_kind: "Body".to_string(),
                    parent_index: parent.index(),
                    child_kind: "Lump".to_string(),
                    child_index: lump_id.index(),
                    detail: "Lump's parent body does not list it as a child".to_string(),
                },
                context: None,
            });
        }
    }

    // Face → Shell upward-pointer check
    for (face_id, face_data) in arena.iter_faces() {
        let shell = face_data.shell();
        arena.get_shell(shell).map_err(|_| {
            KernelError::TopologyViolation {
                err: forge_core::TopologyError::HierarchyViolation {
                    parent_kind: "Shell".to_string(),
                    parent_index: shell.index(),
                    child_kind: "Face".to_string(),
                    child_index: face_id.index(),
                    detail: "Face's parent shell is stale/deleted".to_string(),
                },
                context: None,
            }
        })?;
    }

    // Orphan detection: every shell must be reachable from a region→lump→body chain.
    for (shell_id, shell_data) in arena.iter_shells() {
        let region = shell_data.region();
        let region_data = arena.get_region(region)?;
        let lump = region_data.lump();
        let lump_data = arena.get_lump(lump)?;
        let body = lump_data.body();
        arena.get_body(body).map_err(|_| KernelError::TopologyViolation {
            err: forge_core::TopologyError::HierarchyViolation {
                parent_kind: "Body".to_string(),
                parent_index: body.index(),
                child_kind: "Shell".to_string(),
                child_index: shell_id.index(),
                detail: "Shell is orphaned: its body root is stale/deleted".to_string(),
            },
            context: None,
        })?;
    }

    Ok(())
}
