use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::projection::data::ProjectedTopology;

use super::super::shared::vf;

pub fn validate_projected_acyclic_containment(
    topology: &ProjectedTopology,
) -> Result<(), KernelError> {
    let mut claimed_lumps = BTreeSet::new();
    let mut claimed_regions = BTreeSet::new();
    let mut claimed_shells = BTreeSet::new();
    let mut claimed_faces = BTreeSet::new();

    for (body_index, body) in topology.bodies().iter().enumerate() {
        for lump in &body.lumps {
            if !claimed_lumps.insert(lump.raw()) {
                return Err(vf(
                    "projected_acyclic_containment",
                    format!(
                        "Lump {} is claimed by multiple bodies (or multiple times by body {})",
                        lump.raw(),
                        body_index
                    ),
                ));
            }
        }
    }

    for (lump_index, lump) in topology.lumps().iter().enumerate() {
        for region in &lump.regions {
            if !claimed_regions.insert(region.raw()) {
                return Err(vf(
                    "projected_acyclic_containment",
                    format!(
                        "Region {} is claimed by multiple lumps (or multiple times by lump {})",
                        region.raw(),
                        lump_index
                    ),
                ));
            }
        }
    }

    for (region_index, region) in topology.regions().iter().enumerate() {
        for shell in &region.shells {
            if !claimed_shells.insert(shell.raw()) {
                return Err(vf(
                    "projected_acyclic_containment",
                    format!(
                        "Shell {} is claimed by multiple regions (or multiple times by region {})",
                        shell.raw(),
                        region_index
                    ),
                ));
            }
        }
    }

    for (shell_index, shell) in topology.shells().iter().enumerate() {
        for face in &shell.faces {
            if !claimed_faces.insert(face.raw()) {
                return Err(vf(
                    "projected_acyclic_containment",
                    format!(
                        "Face {} is claimed by multiple shells (or multiple times by shell {})",
                        face.raw(),
                        shell_index
                    ),
                ));
            }
        }
    }

    Ok(())
}
