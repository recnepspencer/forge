use super::{WorthQueryGraphReadAccessRequirementKind, WorthQueryGraphReadAccessRequirementRow};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadAccessRequirementCounters {
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
    domain_operation_capability_registration_count: usize,
}

impl WorthQueryGraphReadAccessRequirementCounters {
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

    pub fn domain_operation_capability_registration_count(&self) -> usize {
        self.domain_operation_capability_registration_count
    }

    pub(crate) fn from_rows(rows: &[WorthQueryGraphReadAccessRequirementRow]) -> Self {
        Self {
            row_count: rows.len(),
            directional_adjacency_count: count_kind(
                rows,
                &WorthQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
            ),
            reverse_adjacency_count: count_kind(
                rows,
                &WorthQueryGraphReadAccessRequirementKind::ReverseAdjacency,
            ),
            predicate_support_count: count_kind(
                rows,
                &WorthQueryGraphReadAccessRequirementKind::PredicateSupport,
            ),
            ordering_support_count: count_kind(
                rows,
                &WorthQueryGraphReadAccessRequirementKind::OrderingSupport,
            ),
            traversal_workset_count: count_kind(
                rows,
                &WorthQueryGraphReadAccessRequirementKind::TraversalWorkset,
            ),
            visited_set_count: count_kind(
                rows,
                &WorthQueryGraphReadAccessRequirementKind::VisitedSet,
            ),
            dedup_set_count: count_kind(rows, &WorthQueryGraphReadAccessRequirementKind::DedupSet),
            workset_count: workset_kind_count(rows),
            buffer_count: count_kind(
                rows,
                &WorthQueryGraphReadAccessRequirementKind::ResultBuffer,
            ),
            proof_support_count: count_kind(
                rows,
                &WorthQueryGraphReadAccessRequirementKind::ProofSupport,
            ),
            materialization_lifecycle_count: count_kind(
                rows,
                &WorthQueryGraphReadAccessRequirementKind::MaterializationLifecycle,
            ),
            live_maintenance_support_count: count_kind(
                rows,
                &WorthQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport,
            ),
            domain_operation_capability_registration_count: count_kind(
                rows,
                &WorthQueryGraphReadAccessRequirementKind::DomainOperationCapabilityRegistration,
            ),
        }
    }
}

fn workset_kind_count(rows: &[WorthQueryGraphReadAccessRequirementRow]) -> usize {
    rows.iter()
        .filter(|row| {
            matches!(
                row.kind(),
                WorthQueryGraphReadAccessRequirementKind::TraversalWorkset
                    | WorthQueryGraphReadAccessRequirementKind::VisitedSet
                    | WorthQueryGraphReadAccessRequirementKind::DedupSet
            )
        })
        .count()
}

fn count_kind(
    rows: &[WorthQueryGraphReadAccessRequirementRow],
    kind: &WorthQueryGraphReadAccessRequirementKind,
) -> usize {
    rows.iter().filter(|row| row.kind() == kind).count()
}
