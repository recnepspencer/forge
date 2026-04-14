use std::collections::BTreeSet;

use crate::data::topology_view::WorthTopologyView;
use crate::validators::error::WorthTopologyValidationError;
use crate::validators::shared::err;

pub fn validate(view: &WorthTopologyView) -> Result<(), WorthTopologyValidationError> {
    validate_radial_presence(view)?;
    validate_radial_cycle_uniqueness(view)?;
    validate_radial_edge_consistency(view)?;
    Ok(())
}

fn validate_radial_presence(view: &WorthTopologyView) -> Result<(), WorthTopologyValidationError> {
    for half_edge in &view.half_edges {
        if half_edge.radial_next_half_edge_id.is_none() {
            return Err(err(
                "radial.radial_presence",
                format!("half-edge {:?} has no radial_next link", half_edge.entity_id),
            ));
        }
    }
    Ok(())
}

fn validate_radial_cycle_uniqueness(
    view: &WorthTopologyView,
) -> Result<(), WorthTopologyValidationError> {
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
    view: &WorthTopologyView,
) -> Result<(), WorthTopologyValidationError> {
    for half_edge in &view.half_edges {
        let edge_id = half_edge.edge_id.ok_or_else(|| {
            err(
                "radial.edge_consistency",
                format!("half-edge {:?} has no edge", half_edge.entity_id),
            )
        })?;
        let radial_next_id = half_edge.radial_next_half_edge_id.ok_or_else(|| {
            err(
                "radial.edge_consistency",
                format!("half-edge {:?} has no radial_next link", half_edge.entity_id),
            )
        })?;
        let radial = view
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

    Ok(())
}
