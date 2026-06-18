use super::candidate_manifest::PlanarBooleanEdgeSplitCloseoutCandidateRow;
use super::decision_localization::PlanarBooleanEdgeSplitCloseoutDecisionRow;
use super::denial::{
    PlanarBooleanEdgeSplitSummumBonumCloseoutDenial as Denial,
    PlanarBooleanEdgeSplitSummumBonumCloseoutDenialKind as Kind,
};
use super::input::PlanarBooleanEdgeSplitSummumBonumCloseoutInput;
use super::source_edge_lineage::PlanarBooleanEdgeSplitCloseoutLineageRow;

pub(crate) fn validate_input(
    input: &PlanarBooleanEdgeSplitSummumBonumCloseoutInput<'_>,
) -> Result<(), Denial> {
    let candidate_counters = input.candidate_index().counters();
    if !input
        .candidate_index()
        .certifies_production_candidate_discovery()
    {
        return Err(Denial::new(
            Kind::CandidateIndexNotProduction,
            "candidate index used fallback",
        ));
    }
    if candidate_counters.query_index_candidate_count() != input.candidate_index().rows().len()
        || candidate_counters.query_index_candidate_count()
            + candidate_counters.query_index_culled_pair_count()
            != candidate_counters.expected_pair_breadth()
    {
        return Err(Denial::new(
            Kind::CandidateIndexCountersDoNotReconcile,
            "candidate index counters do not reconcile rows, emitted candidates, and culled pairs",
        ));
    }
    if !input
        .persistent_naming()
        .certifies_query_native_split_persistent_naming()
    {
        return Err(Denial::new(
            Kind::PersistentNamingNotQueryNative,
            "persistent naming failed",
        ));
    }
    if !input
        .decision_log()
        .certifies_query_native_split_decision_log()
    {
        return Err(Denial::new(
            Kind::DecisionLogNotQueryNative,
            "decision log failed",
        ));
    }
    if !input.split_ledger().certifies_split_edge_chain_ledger() {
        return Err(Denial::new(
            Kind::SplitLedgerNotCertified,
            "split ledger failed",
        ));
    }
    if !input
        .replay_parity()
        .certifies_planar_boolean_replay_parity()
    {
        return Err(Denial::new(
            Kind::ReplayParityNotCertified,
            "replay parity failed",
        ));
    }
    if !input
        .downstream_consumption()
        .certifies_downstream_split_consumption()
    {
        return Err(Denial::new(
            Kind::DownstreamConsumptionNotCertified,
            "downstream failed",
        ));
    }
    if !input
        .loop_reconstruction_consumption()
        .certifies_loop_reconstruction_split_consumption()
    {
        return Err(Denial::new(
            Kind::LoopReconstructionConsumptionNotCertified,
            "loop reconstruction failed",
        ));
    }
    Ok(())
}

pub(crate) fn validate_rows(
    candidate_rows: &[PlanarBooleanEdgeSplitCloseoutCandidateRow],
    lineage_rows: &[PlanarBooleanEdgeSplitCloseoutLineageRow],
    decision_rows: &[PlanarBooleanEdgeSplitCloseoutDecisionRow],
) -> Result<(), Denial> {
    if candidate_rows.iter().any(|row| {
        row.candidate_identity().is_empty()
            || row.left_source_edge_identity().is_empty()
            || row.right_source_edge_identity().is_empty()
            || row.broad_phase_reason().is_empty()
            || row.envelope_basis_identity().is_empty()
            || row.local_frame_identity().is_empty()
            || row.precision_basis_identity().is_empty()
    }) {
        return Err(Denial::new(
            Kind::CandidateRowsMissingProofIdentity,
            "candidate row gap",
        ));
    }
    if lineage_rows.is_empty()
        || lineage_rows.iter().any(|row| {
            row.source_edge_identity().is_empty()
                || row.carrier_identity().is_empty()
                || row.fragment_identities().is_empty()
        })
    {
        return Err(Denial::new(Kind::SplitLineageIncomplete, "lineage row gap"));
    }
    if decision_rows.iter().any(|row| {
        row.decision_identity().is_empty()
            || row.phase_name().is_empty()
            || row.decision_kind_name().is_empty()
            || row.affected_artifact_identity().is_empty()
            || row.upstream_receipt_identity().is_empty()
    }) {
        return Err(Denial::new(
            Kind::DecisionRowsNotLocalized,
            "decision localization gap",
        ));
    }
    Ok(())
}
