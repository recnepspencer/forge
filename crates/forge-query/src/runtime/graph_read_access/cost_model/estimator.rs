use super::{
    ForgeQueryGraphReadAccessCostEstimate, ForgeQueryGraphReadCostAttributionRow,
    ForgeQueryGraphReadCostEstimateCounters, ForgeQueryGraphReadCostEvidence,
    ForgeQueryGraphReadIntrinsicCostContribution, ForgeQueryGraphReadIntrinsicCostEstimate,
    ForgeQueryGraphReadMemoryByteEstimate, ForgeQueryGraphReadSupportedCostContribution,
    ForgeQueryGraphReadSupportedCostEstimate,
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
    let attribution_rows = estimate_cost_attribution_rows(requirements.rows());
    let intrinsic = estimate_intrinsic_cost(&attribution_rows);
    let supported = estimate_supported_cost(&attribution_rows);
    let counters = estimate_cost_counters(requirements.rows());
    ForgeQueryGraphReadAccessCostEstimate::new(
        requirements.digest().as_str(),
        &evidence,
        intrinsic,
        supported,
        counters,
        attribution_rows,
    )
}

fn estimate_intrinsic_cost(
    rows: &[ForgeQueryGraphReadCostAttributionRow],
) -> ForgeQueryGraphReadIntrinsicCostEstimate {
    let frontier_breadth = rows
        .iter()
        .map(|row| row.intrinsic().frontier_breadth())
        .sum::<usize>()
        .max(1);
    let edge_touches = rows
        .iter()
        .map(|row| row.intrinsic().edge_touches())
        .sum::<usize>();
    let candidate_roots = rows
        .iter()
        .map(|row| row.intrinsic().candidate_roots())
        .sum::<usize>()
        .max(1);
    let intermediate_set_size = rows
        .iter()
        .map(|row| row.intrinsic().intermediate_set_size())
        .sum::<usize>();
    ForgeQueryGraphReadIntrinsicCostEstimate::new(
        frontier_breadth,
        edge_touches,
        candidate_roots,
        intermediate_set_size,
    )
}

fn estimate_supported_cost(
    rows: &[ForgeQueryGraphReadCostAttributionRow],
) -> ForgeQueryGraphReadSupportedCostEstimate {
    let mut memory = ForgeQueryGraphReadMemoryByteEstimate::empty();
    let allocation_lifecycle_count = rows
        .iter()
        .map(|row| row.supported().allocation_lifecycle_count())
        .sum::<usize>();
    for row in rows {
        add_memory_contribution(&mut memory, row.supported().memory());
    }
    ForgeQueryGraphReadSupportedCostEstimate::new(memory, allocation_lifecycle_count)
}

fn estimate_cost_attribution_rows(
    rows: &[ForgeQueryGraphReadAccessRequirementRow],
) -> Vec<ForgeQueryGraphReadCostAttributionRow> {
    rows.iter().map(estimate_cost_attribution_row).collect()
}

fn estimate_cost_attribution_row(
    row: &ForgeQueryGraphReadAccessRequirementRow,
) -> ForgeQueryGraphReadCostAttributionRow {
    ForgeQueryGraphReadCostAttributionRow::new(
        row.digest_part(),
        row.kind().clone(),
        intrinsic_contribution(row),
        supported_contribution(row),
    )
}

fn intrinsic_contribution(
    row: &ForgeQueryGraphReadAccessRequirementRow,
) -> ForgeQueryGraphReadIntrinsicCostContribution {
    ForgeQueryGraphReadIntrinsicCostContribution::new(
        relation_frontier_breadth(row).unwrap_or(0),
        relation_edge_touch_estimate(row).unwrap_or(0),
        candidate_root_pressure(row),
        intermediate_set_pressure(row),
    )
}

fn supported_contribution(
    row: &ForgeQueryGraphReadAccessRequirementRow,
) -> ForgeQueryGraphReadSupportedCostContribution {
    let mut memory = ForgeQueryGraphReadMemoryByteEstimate::empty();
    let mut allocation_lifecycle_count = 0;
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
        | ForgeQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport
        | ForgeQueryGraphReadAccessRequirementKind::DomainOperationCapabilityRegistration => {
            allocation_lifecycle_count = 1;
        }
    }
    ForgeQueryGraphReadSupportedCostContribution::new(memory, allocation_lifecycle_count)
}

fn add_memory_contribution(
    target: &mut ForgeQueryGraphReadMemoryByteEstimate,
    contribution: &ForgeQueryGraphReadMemoryByteEstimate,
) {
    target.add_adjacency_bytes(contribution.adjacency_bytes());
    target.add_reverse_adjacency_bytes(contribution.reverse_adjacency_bytes());
    target.add_frontier_bytes(contribution.frontier_bytes());
    target.add_visited_bytes(contribution.visited_bytes());
    target.add_dedup_bytes(contribution.dedup_bytes());
    target.add_predicate_bytes(contribution.predicate_bytes());
    target.add_ordering_bytes(contribution.ordering_bytes());
    target.add_proof_bytes(contribution.proof_bytes());
    target.add_result_bytes(contribution.result_bytes());
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
