use crate::data::graph::SignalGraph;
use crate::data::reuse::ReuseBoundaryFailure;

pub(crate) fn record_reuse_rejection_telemetry(
    graph: &mut SignalGraph,
    failure: &ReuseBoundaryFailure,
) {
    graph.with_telemetry(|telemetry| {
        let evaluation = &mut telemetry.evaluation;
        match failure {
            ReuseBoundaryFailure::UnsupportedStrategyFamily(_) => {
                evaluation.reuse_rejected_unsupported_strategy_count += 1;
            }
            ReuseBoundaryFailure::ContractStrategyDisallowed(_) => {
                evaluation.reuse_rejected_contract_strategy_count += 1;
            }
            ReuseBoundaryFailure::BoundaryMismatch(_)
            | ReuseBoundaryFailure::SnapshotReuseNotAllowed
            | ReuseBoundaryFailure::AuthorityReuseNotAllowed => {
                evaluation.reuse_rejected_boundary_mismatch_count += 1;
            }
            ReuseBoundaryFailure::BoundaryContextUnavailable(_) => {
                evaluation.reuse_rejected_missing_prior_context_count += 1;
            }
            ReuseBoundaryFailure::PersistentCorrespondenceEvidenceMissing => {
                evaluation.reuse_rejected_persistent_correspondence_missing_count += 1;
            }
            ReuseBoundaryFailure::PersistentCorrespondenceEvidenceInvalid => {
                evaluation.reuse_rejected_persistent_correspondence_invalid_count += 1;
            }
            ReuseBoundaryFailure::CompositionRegionLegalityFailure => {
                evaluation.reuse_rejected_composition_region_count += 1;
            }
            ReuseBoundaryFailure::MixedBasisInsufficiency => {
                evaluation.reuse_rejected_mixed_basis_insufficiency_count += 1;
            }
        }
    });
}
