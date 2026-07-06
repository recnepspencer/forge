use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopOverlapChainLineageRow, PlanarBooleanLoopReconstructionLedgerRow,
    PlanarBooleanLoopReconstructionParticipationSupport,
};

pub(super) fn lineage_binds_to_loop(
    ledger_row: &PlanarBooleanLoopReconstructionLedgerRow,
    lineage_row: &PlanarBooleanLoopOverlapChainLineageRow,
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
) -> bool {
    let ledger_fragments = ledger_row
        .fragment_identities()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let lineage_fragments = lineage_row
        .fragment_identities()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if !lineage_fragments.is_empty() && shares_any(&lineage_fragments, &ledger_fragments) {
        return true;
    }

    let lineage_source_edges = lineage_row
        .source_edge_identities()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if lineage_source_edges.is_empty() {
        return false;
    }

    let ledger_source_edges = ledger_source_edge_identities(ledger_row, support);
    !ledger_source_edges.is_empty() && shares_any(&lineage_source_edges, &ledger_source_edges)
}

pub(super) fn ledger_source_edge_identities(
    ledger_row: &PlanarBooleanLoopReconstructionLedgerRow,
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
) -> BTreeSet<String> {
    let fragment_membership_edges = ledger_row
        .fragment_identities()
        .iter()
        .filter_map(|fragment_identity| {
            support
                .fragment_membership_map()
                .membership_for_fragment_identity(fragment_identity)
        })
        .map(|membership| membership.source_edge_identity().to_string())
        .collect::<BTreeSet<_>>();
    if !fragment_membership_edges.is_empty() {
        return fragment_membership_edges;
    }

    support
        .source_loop_carriers()
        .rows()
        .iter()
        .filter(|row| {
            ledger_row
                .source_loop_identities()
                .contains(&row.source_loop_identity().to_string())
        })
        .map(|row| row.source_edge_identity().to_string())
        .collect()
}

pub(super) fn lineage_touches_participating_surface(
    lineage_row: &PlanarBooleanLoopOverlapChainLineageRow,
    participating_source_loop_identities: &BTreeSet<String>,
    participating_fragment_identities: &BTreeSet<String>,
    participating_source_edge_identities: &BTreeSet<String>,
) -> bool {
    lineage_row
        .source_loop_identities()
        .iter()
        .any(|identity| participating_source_loop_identities.contains(identity))
        || lineage_row
            .fragment_identities()
            .iter()
            .any(|identity| participating_fragment_identities.contains(identity))
        || lineage_row
            .source_edge_identities()
            .iter()
            .any(|identity| participating_source_edge_identities.contains(identity))
}

fn shares_any(left: &BTreeSet<String>, right: &BTreeSet<String>) -> bool {
    left.iter().any(|identity| right.contains(identity))
}
