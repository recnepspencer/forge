use std::collections::BTreeSet;

use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::validation::error::TopologyValidationError;
use crate::validation::shared::err;

pub fn validate(view: &InterpretedTopologyView) -> Result<(), TopologyValidationError> {
    let topology = view.materialized().topology();
    validate_radial_presence(view)?;
    validate_radial_cycle_uniqueness(topology)?;
    validate_radial_edge_consistency(topology, view)?;
    Ok(())
}

fn validate_radial_presence(view: &InterpretedTopologyView) -> Result<(), TopologyValidationError> {
    let topology = view.materialized().topology();
    for half_edge in &topology.half_edges {
        if half_edge.radial_next_half_edge_id.is_none() {
            return Err(err(
                "radial.radial_presence",
                format!(
                    "half-edge {:?} has no radial_next link",
                    half_edge.entity_id
                ),
            ));
        }
    }
    Ok(())
}

fn validate_radial_cycle_uniqueness(
    view: &crate::brep::topology_graph::TopologyView,
) -> Result<(), TopologyValidationError> {
    for half_edge in &view.half_edges {
        let mut seen = BTreeSet::new();
        let mut current_id = half_edge.entity_id;

        loop {
            if !seen.insert(current_id) {
                if current_id != half_edge.entity_id {
                    return Err(err(
                        "radial.cycle_uniqueness",
                        format!(
                            "radial walk from {:?} revisits {:?} before closing",
                            half_edge.entity_id, current_id
                        ),
                    ));
                }
                break;
            }

            let current = view
                .half_edges
                .iter()
                .find(|record| record.entity_id == current_id)
                .ok_or_else(|| {
                    err(
                        "radial.cycle_uniqueness",
                        format!("missing radial half-edge {:?}", current_id),
                    )
                })?;

            let next_id = current.radial_next_half_edge_id.ok_or_else(|| {
                err(
                    "radial.cycle_uniqueness",
                    format!("half-edge {:?} is missing radial_next", current.entity_id),
                )
            })?;
            current_id = next_id;
        }
    }
    Ok(())
}

fn validate_radial_edge_consistency(
    topology: &crate::brep::topology_graph::TopologyView,
    interpreted: &InterpretedTopologyView,
) -> Result<(), TopologyValidationError> {
    for half_edge in &topology.half_edges {
        let edge_id = half_edge.edge_id.ok_or_else(|| {
            err(
                "radial.edge_consistency",
                format!("half-edge {:?} has no edge", half_edge.entity_id),
            )
        })?;
        let radial_next_id = half_edge.radial_next_half_edge_id.ok_or_else(|| {
            err(
                "radial.edge_consistency",
                format!(
                    "half-edge {:?} has no radial_next link",
                    half_edge.entity_id
                ),
            )
        })?;
        let radial = topology
            .half_edges
            .iter()
            .find(|record| record.entity_id == radial_next_id)
            .ok_or_else(|| {
                err(
                    "radial.edge_consistency",
                    format!("missing radial half-edge {:?}", radial_next_id),
                )
            })?;
        if radial.edge_id != Some(edge_id) {
            return Err(err(
                "radial.edge_consistency",
                format!(
                    "half-edge {:?} uses edge {:?} but radial neighbor {:?} uses {:?}",
                    half_edge.entity_id, edge_id, radial.entity_id, radial.edge_id
                ),
            ));
        }
    }

    for summary in interpreted.radial_summaries() {
        let shell = topology
            .shells
            .iter()
            .find(|shell| shell.entity_id == summary.shell_id)
            .ok_or_else(|| {
                err(
                    "radial.summary_shell_presence",
                    format!("missing shell {:?} for radial summary", summary.shell_id),
                )
            })?;
        let interpreted_non_manifold = summary.non_manifold_edge_ids.len();
        if interpreted_non_manifold > shell.face_ids.len() {
            return Err(err(
                "radial.summary_consistency",
                format!(
                    "shell {:?} reports {} non-manifold edges across only {} faces",
                    shell.entity_id,
                    interpreted_non_manifold,
                    shell.face_ids.len()
                ),
            ));
        }
    }

    Ok(())
}
