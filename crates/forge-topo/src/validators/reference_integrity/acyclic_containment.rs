//! Acyclic containment hierarchy validator.
//!
//! INVARIANT: The macro-hierarchy (Body -> Lump -> Region -> Shell -> Face)
//! must be a strict Directed Acyclic Graph (DAG) with single ownership.
//! A single Region cannot be claimed by two Lumps. A Shell cannot be claimed
//! by two Regions, etc.

use crate::b_rep::TopologyArena;
use forge_core::KernelError;
use std::collections::BTreeSet;

use super::vf;

pub(crate) fn validate_acyclic_containment(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut claimed_lumps = BTreeSet::new();
    let mut claimed_regions = BTreeSet::new();
    let mut claimed_shells = BTreeSet::new();

    // Because entities can only point "down" to specific types (Body->Lump),
    // structural cycles like Lump->Region->Lump are impossible by Rust type definition.
    // The only remaining DAG violation is multi-parent sharing (diamond or disjoint tree merging).
    // We prevent this by ensuring every child is claimed exactly once during top-down traversal.

    for (body_id, body_data) in arena.iter_bodies() {
        for &lump_id in body_data.lumps() {
            if !claimed_lumps.insert(lump_id.index()) {
                return Err(vf("acyclic_containment", format!(
                    "Lump {} is claimed by multiple Bodies (or multiple times by Body {})",
                    lump_id.index(), body_id.index()
                )));
            }
        }
    }

    for (lump_id, lump_data) in arena.iter_lumps() {
        for &region_id in lump_data.regions() {
            if !claimed_regions.insert(region_id.index()) {
                return Err(vf("acyclic_containment", format!(
                    "Region {} is claimed by multiple Lumps (or multiple times by Lump {})",
                    region_id.index(), lump_id.index()
                )));
            }
        }
    }

    for (region_id, region_data) in arena.iter_regions() {
        for shell_id in region_data.shells() {
            if !claimed_shells.insert(shell_id.index()) {
                return Err(vf("acyclic_containment", format!(
                    "Shell {} is claimed by multiple Regions (or multiple times by Region {})",
                    shell_id.index(), region_id.index()
                )));
            }
        }
    }

    Ok(())
}
