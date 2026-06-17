use super::denial::{
    PlanarBooleanCandidateIndexConsumptionDenial, PlanarBooleanCandidateIndexConsumptionDenialKind,
};
use super::gate::{non_production_fallback_denial, unsupported_lifecycle_denial};
use super::input::PlanarBooleanCandidateIndexConsumptionInput;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanCandidateIndexLifecycleOutcome, PlanarBooleanEventLedgerReceipt,
    PlanarBooleanSegmentCandidateIndexProduct, PlanarBooleanSegmentPairEnumerationReceipt,
};

pub(crate) fn validate_candidate_index_consumption(
    input: &PlanarBooleanCandidateIndexConsumptionInput<'_>,
) -> Result<(), PlanarBooleanCandidateIndexConsumptionDenial> {
    let event_ledger = input.event_ledger();
    let segment_pair_enumeration = input.segment_pair_enumeration();
    require_segment_pair_enumeration_evidence(input, segment_pair_enumeration)?;
    require_event_ledger_evidence(input, event_ledger)?;
    require_event_ledger_binds_segment_pair_enumeration(event_ledger, segment_pair_enumeration)?;
    let product = segment_pair_enumeration.candidate_index_product();
    require_bound_candidate_index_lifecycle(product)?;
    require_production_candidate_index_posture(product)?;
    require_candidate_index_counters_reconcile(product)?;
    Ok(())
}

fn require_segment_pair_enumeration_evidence(
    input: &PlanarBooleanCandidateIndexConsumptionInput<'_>,
    segment_pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
) -> Result<(), PlanarBooleanCandidateIndexConsumptionDenial> {
    input
        .stage_index()
        .require_boolean_receipt(segment_pair_enumeration)
        .map_err(|error| {
            PlanarBooleanCandidateIndexConsumptionDenial::from_evidence_error(
                error,
                segment_pair_enumeration.segment_pair_enumeration_identity(),
            )
        })
}

fn require_event_ledger_evidence(
    input: &PlanarBooleanCandidateIndexConsumptionInput<'_>,
    event_ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanCandidateIndexConsumptionDenial> {
    input
        .stage_index()
        .require_boolean_receipt(event_ledger)
        .map_err(|error| {
            PlanarBooleanCandidateIndexConsumptionDenial::from_evidence_error(
                error,
                event_ledger.event_ledger_identity(),
            )
        })
}

fn require_event_ledger_binds_segment_pair_enumeration(
    event_ledger: &PlanarBooleanEventLedgerReceipt,
    segment_pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
) -> Result<(), PlanarBooleanCandidateIndexConsumptionDenial> {
    if event_ledger.segment_pair_enumeration_identity()
        != segment_pair_enumeration.segment_pair_enumeration_identity()
    {
        return Err(PlanarBooleanCandidateIndexConsumptionDenial::new(
            PlanarBooleanCandidateIndexConsumptionDenialKind::EventLedgerSegmentPairEnumerationMismatch,
            event_ledger.event_ledger_identity(),
            "event ledger must consume the same segment-pair enumeration receipt that owns the candidate-index product",
        ));
    }
    Ok(())
}

fn require_bound_candidate_index_lifecycle(
    product: &PlanarBooleanSegmentCandidateIndexProduct,
) -> Result<(), PlanarBooleanCandidateIndexConsumptionDenial> {
    if product.lifecycle_outcome() != PlanarBooleanCandidateIndexLifecycleOutcome::Bound {
        return Err(unsupported_lifecycle_denial(product.product_identity()));
    }
    Ok(())
}

fn require_production_candidate_index_posture(
    product: &PlanarBooleanSegmentCandidateIndexProduct,
) -> Result<(), PlanarBooleanCandidateIndexConsumptionDenial> {
    if !product.certifies_production_candidate_discovery() {
        return Err(non_production_fallback_denial(product.product_identity()));
    }
    Ok(())
}

fn require_candidate_index_counters_reconcile(
    product: &PlanarBooleanSegmentCandidateIndexProduct,
) -> Result<(), PlanarBooleanCandidateIndexConsumptionDenial> {
    let counters = product.counters();
    if counters.emitted_pair_breadth() != product.rows().len()
        || counters.query_index_candidate_count() != product.rows().len()
        || counters
            .query_index_candidate_count()
            .saturating_add(counters.query_index_culled_pair_count())
            != counters.expected_pair_breadth()
    {
        return Err(PlanarBooleanCandidateIndexConsumptionDenial::new(
            PlanarBooleanCandidateIndexConsumptionDenialKind::CandidateIndexCounterMismatch,
            product.product_identity(),
            "candidate-index product rows, emitted counters, and culled-pair counters must reconcile before split consumption",
        ));
    }
    Ok(())
}
