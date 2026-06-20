use super::{
    ForgeQueryGraphReadAccessCostEstimate, ForgeQueryGraphReadCostEstimateCounters,
    ForgeQueryGraphReadCostEvidence, ForgeQueryGraphReadIntrinsicCostEstimate,
    ForgeQueryGraphReadMemoryByteEstimate, ForgeQueryGraphReadSupportedCostEstimate,
};
use crate::runtime::{
    ForgeQueryGraphReadAccessRequirementKind, ForgeQueryGraphReadAccessRequirementRow,
    ForgeQueryGraphReadAccessRequirementSet, ForgeQueryGraphReadPredicateFamily,
    ForgeQueryGraphReadResultPressure,
};

const UNKNOWN_RELATION_TOUCH_UNIT: usize = 32;
const UNKNOWN_FRONTIER_UNIT_BYTES: usize = 256;
const UNKNOWN_SET_UNIT_BYTES: usize = 192;
const UNKNOWN_FIELD_SUPPORT_BYTES: usize = 512;
const UNKNOWN_PROOF_BYTES: usize = 256;

pub fn estimate_graph_read_access_cost(
    requirements: &ForgeQueryGraphReadAccessRequirementSet,
    evidence: ForgeQueryGraphReadCostEvidence,
) -> ForgeQueryGraphReadAccessCostEstimate {
    let intrinsic = estimate_intrinsic_cost(requirements.rows());
    let supported = estimate_supported_cost(requirements.rows());
    let counters = estimate_cost_counters(requirements.rows());
    ForgeQueryGraphReadAccessCostEstimate::new(
        requirements.digest().as_str(),
        &evidence,
        intrinsic,
        supported,
        counters,
    )
}

fn estimate_intrinsic_cost(
    rows: &[ForgeQueryGraphReadAccessRequirementRow],
) -> ForgeQueryGraphReadIntrinsicCostEstimate {
    let frontier_breadth = rows
        .iter()
        .filter_map(relation_frontier_breadth)
        .sum::<usize>()
        .max(1);
    let edge_touches = rows
        .iter()
        .filter_map(relation_edge_touch_estimate)
        .sum::<usize>();
    let candidate_roots = rows
        .iter()
        .map(candidate_root_pressure)
        .sum::<usize>()
        .max(1);
    let intermediate_set_size = rows.iter().map(intermediate_set_pressure).sum::<usize>();
    ForgeQueryGraphReadIntrinsicCostEstimate::new(
        frontier_breadth,
        edge_touches,
        candidate_roots,
        intermediate_set_size,
    )
}

fn estimate_supported_cost(
    rows: &[ForgeQueryGraphReadAccessRequirementRow],
) -> ForgeQueryGraphReadSupportedCostEstimate {
    let mut memory = ForgeQueryGraphReadMemoryByteEstimate::default();
    let mut allocation_lifecycle_count = 0;
    for row in rows {
        match row.kind() {
            ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency => {
                memory.add_adjacency_bytes(relation_memory_bytes(row));
            }
            ForgeQueryGraphReadAccessRequirementKind::ReverseAdjacency => {
                memory.add_reverse_adjacency_bytes(relation_memory_bytes(row));
            }
            ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset => {
                memory.add_frontier_bytes(frontier_memory_bytes(row));
            }
            ForgeQueryGraphReadAccessRequirementKind::VisitedSet => {
                memory.add_visited_bytes(set_memory_bytes(row));
            }
            ForgeQueryGraphReadAccessRequirementKind::DedupSet => {
                memory.add_dedup_bytes(set_memory_bytes(row));
            }
            ForgeQueryGraphReadAccessRequirementKind::PredicateSupport => {
                memory.add_predicate_bytes(predicate_memory_bytes(row));
            }
            ForgeQueryGraphReadAccessRequirementKind::OrderingSupport => {
                memory.add_ordering_bytes(ordering_memory_bytes(row));
            }
            ForgeQueryGraphReadAccessRequirementKind::ProofSupport => {
                memory.add_proof_bytes(UNKNOWN_PROOF_BYTES);
            }
            ForgeQueryGraphReadAccessRequirementKind::ResultBuffer => {
                memory.add_result_bytes(result_memory_bytes(row));
            }
            ForgeQueryGraphReadAccessRequirementKind::MaterializationLifecycle
            | ForgeQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport => {
                allocation_lifecycle_count += 1;
            }
        }
    }
    ForgeQueryGraphReadSupportedCostEstimate::new(memory, allocation_lifecycle_count)
}

fn estimate_cost_counters(
    rows: &[ForgeQueryGraphReadAccessRequirementRow],
) -> ForgeQueryGraphReadCostEstimateCounters {
    let estimated_relation_row_count = rows
        .iter()
        .filter(|row| row.relation_name().is_some())
        .count();
    let estimated_workset_row_count = rows
        .iter()
        .filter(|row| {
            matches!(
                row.kind(),
                ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset
                    | ForgeQueryGraphReadAccessRequirementKind::VisitedSet
                    | ForgeQueryGraphReadAccessRequirementKind::DedupSet
            )
        })
        .count();
    let estimated_buffer_row_count = rows
        .iter()
        .filter(|row| row.kind() == &ForgeQueryGraphReadAccessRequirementKind::ResultBuffer)
        .count();
    ForgeQueryGraphReadCostEstimateCounters::new(
        rows.len(),
        estimated_relation_row_count,
        estimated_workset_row_count,
        estimated_buffer_row_count,
    )
}

fn relation_frontier_breadth(row: &ForgeQueryGraphReadAccessRequirementRow) -> Option<usize> {
    row.relation_name()
        .map(|_| row.relation_depth().unwrap_or(1).max(1) * UNKNOWN_RELATION_TOUCH_UNIT)
}

fn relation_edge_touch_estimate(row: &ForgeQueryGraphReadAccessRequirementRow) -> Option<usize> {
    row.relation_name()
        .map(|_| row.relation_depth().unwrap_or(1).max(1) * UNKNOWN_RELATION_TOUCH_UNIT)
}

fn candidate_root_pressure(row: &ForgeQueryGraphReadAccessRequirementRow) -> usize {
    match row.predicate_family() {
        Some(ForgeQueryGraphReadPredicateFamily::None) | None => 0,
        Some(ForgeQueryGraphReadPredicateFamily::Equality) => 4,
        Some(ForgeQueryGraphReadPredicateFamily::Range)
        | Some(ForgeQueryGraphReadPredicateFamily::Membership) => 8,
        Some(ForgeQueryGraphReadPredicateFamily::Text)
        | Some(ForgeQueryGraphReadPredicateFamily::Presence)
        | Some(ForgeQueryGraphReadPredicateFamily::Mixed) => 16,
    }
}

fn intermediate_set_pressure(row: &ForgeQueryGraphReadAccessRequirementRow) -> usize {
    match row.kind() {
        ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset
        | ForgeQueryGraphReadAccessRequirementKind::VisitedSet
        | ForgeQueryGraphReadAccessRequirementKind::DedupSet => {
            row.relation_depth().unwrap_or(1).max(1) * UNKNOWN_RELATION_TOUCH_UNIT
        }
        _ => 0,
    }
}

fn relation_memory_bytes(row: &ForgeQueryGraphReadAccessRequirementRow) -> usize {
    row.relation_depth().unwrap_or(1).max(1) * UNKNOWN_RELATION_TOUCH_UNIT * 8
}

fn frontier_memory_bytes(row: &ForgeQueryGraphReadAccessRequirementRow) -> usize {
    row.relation_depth().unwrap_or(1).max(1) * UNKNOWN_FRONTIER_UNIT_BYTES
}

fn set_memory_bytes(row: &ForgeQueryGraphReadAccessRequirementRow) -> usize {
    row.relation_depth().unwrap_or(1).max(1) * UNKNOWN_SET_UNIT_BYTES
}

fn predicate_memory_bytes(row: &ForgeQueryGraphReadAccessRequirementRow) -> usize {
    row.predicate_field_authorities().len().max(1) * UNKNOWN_FIELD_SUPPORT_BYTES
}

fn ordering_memory_bytes(row: &ForgeQueryGraphReadAccessRequirementRow) -> usize {
    row.ordering_field_authorities().len().max(1) * UNKNOWN_FIELD_SUPPORT_BYTES
}

fn result_memory_bytes(row: &ForgeQueryGraphReadAccessRequirementRow) -> usize {
    match row.result_pressure() {
        Some(ForgeQueryGraphReadResultPressure::Detail) => 256,
        Some(ForgeQueryGraphReadResultPressure::CollectionNarrow) => 1024,
        Some(ForgeQueryGraphReadResultPressure::CollectionWide) | None => 2048,
    }
}
