#[cfg(test)]
use std::collections::BTreeSet;

#[cfg(test)]
use forge_relational::facade::identity::EntityId;
use serde::Serialize;

use super::TraversalViewsSourceRow;
#[cfg(test)]
use crate::brep::topology_graph::TopologyView;
#[cfg(test)]
use crate::derived_topology::invalidation_plan::migrated_products::traversal_views::TraversalViewsMigrationError;
#[cfg(test)]
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraversalViewsReadSource {
    selected_rows: Vec<TraversalViewsSourceRow>,
    available_traversal_count: usize,
    source_digest: String,
}

impl TraversalViewsReadSource {
    #[cfg(test)]
    pub(crate) fn select_from_touched_closure(
        selected_plan: &DerivedInvalidationSelectedPlan,
        touched_closure: &DerivedInvalidationTouchedClosure,
        topology: &TopologyView,
    ) -> Result<Self, TraversalViewsMigrationError> {
        if selected_plan.touched_closure_digest() != touched_closure.closure_digest() {
            return Err(
                TraversalViewsMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan,
            );
        }

        let traversal_rows = available_traversal_source_rows_from_topology(topology);
        let available_traversal_count = traversal_rows.len();
        let touched_entities = touched_traversal_anchor_entity_set(touched_closure);
        let selected_rows = traversal_rows
            .into_iter()
            .filter(|row| touched_entities.contains(&row.anchor_entity_id()))
            .collect::<Vec<_>>();
        if selected_rows.is_empty() {
            return Err(
                TraversalViewsMigrationError::ReadStageTouchedClosureSelectedNoTraversalRows,
            );
        }

        Self::from_rows(selected_rows, available_traversal_count)
    }

    #[cfg(test)]
    pub(crate) fn from_topology_view_with_selected_prefix(
        topology: &TopologyView,
        selected_traversal_count: usize,
    ) -> Result<Self, TraversalViewsMigrationError> {
        let traversal_rows = available_traversal_source_rows_from_topology(topology);
        let available_traversal_count = traversal_rows.len();
        if selected_traversal_count > available_traversal_count {
            return Err(TraversalViewsMigrationError::ReadStageSelectedRowsExceedAvailableRows);
        }
        Self::from_rows(
            traversal_rows
                .into_iter()
                .take(selected_traversal_count)
                .collect(),
            available_traversal_count,
        )
    }

    #[cfg(test)]
    pub(crate) fn from_rows(
        selected_rows: Vec<TraversalViewsSourceRow>,
        available_traversal_count: usize,
    ) -> Result<Self, TraversalViewsMigrationError> {
        if selected_rows.len() > available_traversal_count {
            return Err(TraversalViewsMigrationError::ReadStageSelectedRowsExceedAvailableRows);
        }
        let source_digest = traversal_read_source_digest(&selected_rows, available_traversal_count);
        Ok(Self {
            selected_rows,
            available_traversal_count,
            source_digest,
        })
    }

    pub fn selected_rows(&self) -> &[TraversalViewsSourceRow] {
        &self.selected_rows
    }

    pub const fn available_traversal_count(&self) -> usize {
        self.available_traversal_count
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }
}

#[cfg(test)]
fn available_traversal_source_rows_from_topology(
    topology: &TopologyView,
) -> Vec<TraversalViewsSourceRow> {
    let mut rows = Vec::new();
    rows.extend(topology.faces.iter().map(|face| {
        TraversalViewsSourceRow::new(
            "face.boundary_walk",
            face.entity_id,
            face.boundary_half_edge_ids.len(),
        )
    }));
    rows.extend(topology.loops.iter().map(|loop_row| {
        TraversalViewsSourceRow::new(
            "loop.half_edge_walk",
            loop_row.entity_id,
            loop_row.half_edge_ids.len(),
        )
    }));
    rows.extend(topology.wires.iter().map(|wire| {
        TraversalViewsSourceRow::new(
            "wire.half_edge_walk",
            wire.entity_id,
            wire.half_edge_ids.len(),
        )
    }));
    rows.extend(topology.shells.iter().map(|shell| {
        TraversalViewsSourceRow::new("shell.face_walk", shell.entity_id, shell.face_ids.len())
    }));
    rows
}

#[cfg(test)]
fn touched_traversal_anchor_entity_set(
    touched_closure: &DerivedInvalidationTouchedClosure,
) -> BTreeSet<EntityId> {
    touched_closure
        .basis()
        .entities()
        .iter()
        .map(|entity| entity.entity_id())
        .collect()
}

#[cfg(test)]
fn traversal_read_source_digest(
    selected_rows: &[TraversalViewsSourceRow],
    available_traversal_count: usize,
) -> String {
    let mut parts = vec![
        "worth-topo:traversal-views-read-source:v1".to_string(),
        format!("selected-traversals:{}", selected_rows.len()),
        format!("available-traversals:{available_traversal_count}"),
    ];
    parts.extend(
        selected_rows
            .iter()
            .map(|row| format!("source-row:{}", row.row_digest())),
    );
    super::super::super::super::catalog::catalog_digest(parts)
}
