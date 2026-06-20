use super::{ForgeQueryGraphReadAccessRequirementKind, ForgeQueryGraphReadAccessRequirementRow};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessRequirementCounters {
    row_count: usize,
    directional_adjacency_count: usize,
    reverse_adjacency_count: usize,
    predicate_support_count: usize,
    ordering_support_count: usize,
    traversal_workset_count: usize,
    visited_set_count: usize,
    dedup_set_count: usize,
    workset_count: usize,
    buffer_count: usize,
    proof_support_count: usize,
    materialization_lifecycle_count: usize,
    live_maintenance_support_count: usize,
}

impl ForgeQueryGraphReadAccessRequirementCounters {
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn directional_adjacency_count(&self) -> usize {
        self.directional_adjacency_count
    }

    pub fn reverse_adjacency_count(&self) -> usize {
        self.reverse_adjacency_count
    }

    pub fn predicate_support_count(&self) -> usize {
        self.predicate_support_count
    }

    pub fn ordering_support_count(&self) -> usize {
        self.ordering_support_count
    }

    pub fn traversal_workset_count(&self) -> usize {
        self.traversal_workset_count
    }

    pub fn visited_set_count(&self) -> usize {
        self.visited_set_count
    }

    pub fn dedup_set_count(&self) -> usize {
        self.dedup_set_count
    }

    pub fn workset_count(&self) -> usize {
        self.workset_count
    }

    pub fn buffer_count(&self) -> usize {
        self.buffer_count
    }

    pub fn proof_support_count(&self) -> usize {
        self.proof_support_count
    }

    pub fn materialization_lifecycle_count(&self) -> usize {
        self.materialization_lifecycle_count
    }

    pub fn live_maintenance_support_count(&self) -> usize {
        self.live_maintenance_support_count
    }

    pub(crate) fn from_rows(rows: &[ForgeQueryGraphReadAccessRequirementRow]) -> Self {
        Self {
            row_count: rows.len(),
            directional_adjacency_count: count_kind(
                rows,
                &ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
            ),
            reverse_adjacency_count: count_kind(
                rows,
                &ForgeQueryGraphReadAccessRequirementKind::ReverseAdjacency,
            ),
            predicate_support_count: count_kind(
                rows,
                &ForgeQueryGraphReadAccessRequirementKind::PredicateSupport,
            ),
            ordering_support_count: count_kind(
                rows,
                &ForgeQueryGraphReadAccessRequirementKind::OrderingSupport,
            ),
            traversal_workset_count: count_kind(
                rows,
                &ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset,
            ),
            visited_set_count: count_kind(
                rows,
                &ForgeQueryGraphReadAccessRequirementKind::VisitedSet,
            ),
            dedup_set_count: count_kind(rows, &ForgeQueryGraphReadAccessRequirementKind::DedupSet),
            workset_count: workset_kind_count(rows),
            buffer_count: count_kind(
                rows,
                &ForgeQueryGraphReadAccessRequirementKind::ResultBuffer,
            ),
            proof_support_count: count_kind(
                rows,
                &ForgeQueryGraphReadAccessRequirementKind::ProofSupport,
            ),
            materialization_lifecycle_count: count_kind(
                rows,
                &ForgeQueryGraphReadAccessRequirementKind::MaterializationLifecycle,
            ),
            live_maintenance_support_count: count_kind(
                rows,
                &ForgeQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport,
            ),
        }
    }
}

fn workset_kind_count(rows: &[ForgeQueryGraphReadAccessRequirementRow]) -> usize {
    rows.iter()
        .filter(|row| {
            matches!(
                row.kind(),
                ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset
                    | ForgeQueryGraphReadAccessRequirementKind::VisitedSet
                    | ForgeQueryGraphReadAccessRequirementKind::DedupSet
            )
        })
        .count()
}

fn count_kind(
    rows: &[ForgeQueryGraphReadAccessRequirementRow],
    kind: &ForgeQueryGraphReadAccessRequirementKind,
) -> usize {
    rows.iter().filter(|row| row.kind() == kind).count()
}
