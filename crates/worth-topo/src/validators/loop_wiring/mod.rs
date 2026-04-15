use std::collections::BTreeSet;

use forge_relational::facade::identity::EntityId;

use crate::data::topology_view::WorthTopologyView;
use crate::materialization::MaterializedTopologyView;
use crate::validators::error::WorthTopologyValidationError;
use crate::validators::shared::err;

pub fn validate(view: &MaterializedTopologyView) -> Result<(), WorthTopologyValidationError> {
    let view = view.topology();
    validate_loop_membership(view)?;
    validate_prev_next_symmetry(view)?;
    validate_loop_cardinality(view)?;
    validate_no_duplicate_half_edges_in_loop(view)?;
    Ok(())
}

fn validate_loop_membership(view: &WorthTopologyView) -> Result<(), WorthTopologyValidationError> {
    for loop_record in &view.loops {
        if loop_record.half_edge_ids.is_empty() {
            return Err(err(
                "loop_wiring.loop_membership",
                format!("loop {:?} contains no half-edges", loop_record.entity_id),
            ));
        }
        for half_edge_id in &loop_record.half_edge_ids {
            let Some(half_edge) = view.half_edges.iter().find(|record| record.entity_id == *half_edge_id) else {
                return Err(err(
                    "loop_wiring.loop_membership",
                    format!("loop {:?} references missing half-edge {:?}", loop_record.entity_id, half_edge_id),
                ));
            };
            if half_edge.loop_id != Some(loop_record.entity_id) {
                return Err(err(
                    "loop_wiring.loop_membership",
                    format!(
                        "half-edge {:?} is listed in loop {:?} but records loop {:?}",
                        half_edge.entity_id,
                        loop_record.entity_id,
                        half_edge.loop_id
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn validate_prev_next_symmetry(view: &WorthTopologyView) -> Result<(), WorthTopologyValidationError> {
    for half_edge in &view.half_edges {
        let Some(next_id) = half_edge.next_half_edge_id else {
            return Err(err("loop_wiring.prev_next_symmetry", format!("half-edge {:?} has no next", half_edge.entity_id)));
        };
        let Some(prev_id) = half_edge.prev_half_edge_id else {
            return Err(err("loop_wiring.prev_next_symmetry", format!("half-edge {:?} has no prev", half_edge.entity_id)));
        };

        let next = lookup_half_edge(view, next_id, "loop_wiring.prev_next_symmetry")?;
        let prev = lookup_half_edge(view, prev_id, "loop_wiring.prev_next_symmetry")?;

        if next.prev_half_edge_id != Some(half_edge.entity_id) {
            return Err(err(
                "loop_wiring.prev_next_symmetry",
                format!("next link from {:?} to {:?} is not reciprocated", half_edge.entity_id, next_id),
            ));
        }
        if prev.next_half_edge_id != Some(half_edge.entity_id) {
            return Err(err(
                "loop_wiring.prev_next_symmetry",
                format!("prev link from {:?} to {:?} is not reciprocated", half_edge.entity_id, prev_id),
            ));
        }
    }
    Ok(())
}

fn validate_loop_cardinality(view: &WorthTopologyView) -> Result<(), WorthTopologyValidationError> {
    for loop_record in &view.loops {
        if loop_record.half_edge_ids.len() < 1 {
            return Err(err(
                "loop_wiring.loop_cardinality",
                format!("loop {:?} has invalid cardinality {}", loop_record.entity_id, loop_record.half_edge_ids.len()),
            ));
        }
    }
    Ok(())
}

fn validate_no_duplicate_half_edges_in_loop(
    view: &WorthTopologyView,
) -> Result<(), WorthTopologyValidationError> {
    for loop_record in &view.loops {
        let mut seen = BTreeSet::new();
        for half_edge_id in &loop_record.half_edge_ids {
            if !seen.insert(*half_edge_id) {
                return Err(err(
                    "loop_wiring.duplicate_half_edges",
                    format!("loop {:?} references half-edge {:?} more than once", loop_record.entity_id, half_edge_id),
                ));
            }
        }
    }
    Ok(())
}

fn lookup_half_edge<'a>(
    view: &'a WorthTopologyView,
    entity_id: EntityId,
    validator: &'static str,
) -> Result<&'a crate::data::topology_view::WorthTopologyHalfEdge, WorthTopologyValidationError> {
    view.half_edges
        .iter()
        .find(|record| record.entity_id == entity_id)
        .ok_or_else(|| err(validator, format!("missing half-edge {:?}", entity_id)))
}
