use super::denial::{PlanarBooleanEdgeSplitRequestDenial, PlanarBooleanEdgeSplitRequestDenialKind};
use super::input::PlanarBooleanEdgeSplitRequestInput;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanCandidateIndexConsumptionGate;
use crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerReceipt;

pub(crate) fn validate_edge_split_request(
    input: &PlanarBooleanEdgeSplitRequestInput<'_>,
) -> Result<(), PlanarBooleanEdgeSplitRequestDenial> {
    let event_ledger = input.event_ledger();
    let candidate_index_gate = input.candidate_index_gate();
    require_event_ledger_evidence(input, event_ledger)?;
    require_gate_binds_event_ledger(candidate_index_gate, event_ledger)?;
    require_gate_binds_downstream_consumption(candidate_index_gate, event_ledger)?;
    require_gate_binds_reduced_pair(candidate_index_gate, event_ledger)?;
    require_gate_binds_segment_pair_enumeration(candidate_index_gate, event_ledger)?;
    require_production_candidate_index_gate(candidate_index_gate)
}

fn require_event_ledger_evidence(
    input: &PlanarBooleanEdgeSplitRequestInput<'_>,
    event_ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanEdgeSplitRequestDenial> {
    input
        .stage_index()
        .require_boolean_receipt(event_ledger)
        .map_err(|error| {
            PlanarBooleanEdgeSplitRequestDenial::from_event_ledger_evidence_error(
                error,
                event_ledger.event_ledger_identity(),
            )
        })
}

fn require_gate_binds_event_ledger(
    candidate_index_gate: &PlanarBooleanCandidateIndexConsumptionGate,
    event_ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanEdgeSplitRequestDenial> {
    if candidate_index_gate.event_ledger_identity() != event_ledger.event_ledger_identity() {
        return Err(PlanarBooleanEdgeSplitRequestDenial::new(
            PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateEventLedgerMismatch,
            event_ledger.event_ledger_identity(),
            "edge split request must consume the same event ledger proven by the candidate-index gate",
        ));
    }
    Ok(())
}

fn require_gate_binds_downstream_consumption(
    candidate_index_gate: &PlanarBooleanCandidateIndexConsumptionGate,
    event_ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanEdgeSplitRequestDenial> {
    if candidate_index_gate.downstream_consumption_identity()
        != event_ledger.downstream_consumption_identity()
    {
        return Err(PlanarBooleanEdgeSplitRequestDenial::new(
            PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateDownstreamMismatch,
            event_ledger.downstream_consumption_identity(),
            "edge split request must preserve the event ledger downstream-consumption identity",
        ));
    }
    Ok(())
}

fn require_gate_binds_reduced_pair(
    candidate_index_gate: &PlanarBooleanCandidateIndexConsumptionGate,
    event_ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanEdgeSplitRequestDenial> {
    if candidate_index_gate.reduced_pair_identity() != event_ledger.reduced_pair_identity() {
        return Err(PlanarBooleanEdgeSplitRequestDenial::new(
            PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateReducedPairMismatch,
            event_ledger.reduced_pair_identity(),
            "edge split request must preserve the reduced operand-pair identity",
        ));
    }
    Ok(())
}

fn require_gate_binds_segment_pair_enumeration(
    candidate_index_gate: &PlanarBooleanCandidateIndexConsumptionGate,
    event_ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanEdgeSplitRequestDenial> {
    if candidate_index_gate.segment_pair_enumeration_identity()
        != event_ledger.segment_pair_enumeration_identity()
    {
        return Err(PlanarBooleanEdgeSplitRequestDenial::new(
            PlanarBooleanEdgeSplitRequestDenialKind::CandidateIndexGateSegmentPairMismatch,
            event_ledger.segment_pair_enumeration_identity(),
            "edge split request must preserve the segment-pair enumeration identity",
        ));
    }
    Ok(())
}

fn require_production_candidate_index_gate(
    candidate_index_gate: &PlanarBooleanCandidateIndexConsumptionGate,
) -> Result<(), PlanarBooleanEdgeSplitRequestDenial> {
    if !candidate_index_gate.certifies_production_candidate_discovery() {
        return Err(PlanarBooleanEdgeSplitRequestDenial::new(
            PlanarBooleanEdgeSplitRequestDenialKind::NonProductionCandidateIndexGate,
            candidate_index_gate.gate_identity(),
            "edge split request requires a production Query-owned candidate-index gate",
        ));
    }
    Ok(())
}
