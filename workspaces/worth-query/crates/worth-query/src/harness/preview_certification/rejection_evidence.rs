use crate::harness::fixtures::{
    execution_preflights,
    preview_bridge::{
        active_preview_artifacts, declared_preview_session, discarded_preview_artifacts,
        promoted_preview_artifacts, promoted_preview_replay_bundle,
    },
};
use crate::preview::{
    admit_authoritative_preview_comparison_candidate, admit_preview_promotion_parity_comparison,
    admit_promotion_eligible_preview_session_plan_binding, admit_scoped_preview_live_session_plan,
    admit_scoped_preview_session_plan_binding_from_preview_binding, assess_preview_live_drift,
    bind_preflight_to_preview_session, execute_promotion_eligible_preview_session_plan,
    PreviewEvaluationClass, PreviewLiveDriftOutcome, PreviewSessionQueryContext,
    PreviewWorkflowFoundationRequest,
};

use super::model::PreviewCertificationRejection;

pub(super) struct PreviewRejectionEvidence {
    pub(super) unsupported_preview_family: PreviewCertificationRejection,
    pub(super) invalid_basis: PreviewCertificationRejection,
    pub(super) stale_lifecycle: PreviewCertificationRejection,
    pub(super) discarded_lifecycle: PreviewCertificationRejection,
    pub(super) preview_live_drift_denied: PreviewCertificationRejection,
    pub(super) preview_live_broad_fallback_denied: PreviewCertificationRejection,
    pub(super) read_only_writeback_foundation_denied: PreviewCertificationRejection,
    pub(super) promotion_linkage_denied: PreviewCertificationRejection,
    pub(super) replay_linkage_denied: PreviewCertificationRejection,
    pub(super) shape_mismatch_denied: PreviewCertificationRejection,
}

pub(super) fn build_rejection_evidence() -> PreviewRejectionEvidence {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) = active_preview_artifacts("preview-certification");
    let active_binding = bind_preflight_to_preview_session(
        preflight.clone(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("rejection evidence preview binding should succeed");
    let promotable_binding = bind_preflight_to_preview_session(
        preflight.clone(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("rejection evidence promotable preview binding should succeed");
    let preview_live_binding = admit_scoped_preview_live_session_plan(
        admit_scoped_preview_session_plan_binding_from_preview_binding(promotable_binding)
            .expect("rejection evidence should derive preview-live binding"),
        crate::live::promote_preflight_bundle_to_live(&preflight)
            .expect("rejection evidence should reuse live proof"),
    )
    .expect("rejection evidence preview-live admission should succeed");

    let (_invalid_runtime, _invalid_active, foreign_execution_record) =
        active_preview_artifacts("preview-certification-invalid-basis");
    let unsupported_preview_family = bind_preflight_to_preview_session(
        execution_preflights::cdc_collection_preflight(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect_err("unsupported preview family should reject");
    let invalid_basis = bind_preflight_to_preview_session(
        execution_preflights::direct_runtime_preflight(),
        PreviewSessionQueryContext::active(
            &active,
            &foreign_execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect_err("foreign execution record should reject as invalid basis");
    let (_declared_runtime, declared) = declared_preview_session("preview-certification-declared");
    let stale_lifecycle = bind_preflight_to_preview_session(
        execution_preflights::direct_runtime_preflight(),
        PreviewSessionQueryContext::declared(&declared, PreviewEvaluationClass::read_only()),
    )
    .expect_err("declared lifecycle should reject");
    let (_discarded_runtime, discarded, _discard_record) =
        discarded_preview_artifacts("preview-certification-discarded");
    let discarded_lifecycle = bind_preflight_to_preview_session(
        execution_preflights::direct_runtime_preflight(),
        PreviewSessionQueryContext::discarded(&discarded, PreviewEvaluationClass::read_only()),
    )
    .expect_err("discarded lifecycle should reject");
    let preview_live_drift_denied = match assess_preview_live_drift(
        &preview_live_binding,
        PreviewSessionQueryContext::discarded(
            &discarded,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    ) {
        PreviewLiveDriftOutcome::DriftDenied(denied) => denied,
        other => panic!("discarded preview-live should deny drift, got {other:?}"),
    };
    let preview_live_broad_fallback_denied = match assess_preview_live_drift(
        &preview_live_binding,
        PreviewSessionQueryContext::active(
            &active,
            &foreign_execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    ) {
        PreviewLiveDriftOutcome::DriftDenied(denied) => denied,
        other => panic!("preview-live broad fallback should deny drift, got {other:?}"),
    };
    let (_promoted_runtime, _promoted, _promoted_execution, promotion_record) =
        promoted_preview_artifacts("preview-certification-promotion-linkage");
    let promotion_linkage_denied = bind_preflight_to_preview_session(
        execution_preflights::direct_runtime_preflight(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        )
        .with_promotion_record(&promotion_record),
    )
    .expect_err("promotion linkage should reject");
    let (
        _replay_runtime,
        _replay_promoted,
        _replay_execution,
        _replay_promotion_record,
        replay_bundle,
    ) = promoted_preview_replay_bundle("preview-certification-replay-linkage");
    let replay_linkage_denied = bind_preflight_to_preview_session(
        execution_preflights::direct_runtime_preflight(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        )
        .with_replay_bundle(&replay_bundle),
    )
    .expect_err("replay linkage should reject");
    let shape_mismatch_preview_binding = bind_preflight_to_preview_session(
        execution_preflights::ordered_collection_without_traversal_preflight(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("shape mismatch preview binding should succeed");
    let shape_mismatch_preview_execution = execute_promotion_eligible_preview_session_plan(
        &admit_promotion_eligible_preview_session_plan_binding(shape_mismatch_preview_binding)
            .expect("shape mismatch promotion binding should admit"),
    )
    .expect("shape mismatch preview execution should succeed");
    let shape_mismatch_candidate_preflight = execution_preflights::ordered_collection_preflight();
    let shape_mismatch_candidate_execution =
        crate::execution::execute_preflight_bundle(&shape_mismatch_candidate_preflight)
            .expect("shape mismatch candidate execution should succeed");
    let shape_mismatch_candidate = admit_authoritative_preview_comparison_candidate(
        &shape_mismatch_candidate_preflight,
        &shape_mismatch_candidate_execution,
    )
    .expect("shape mismatch candidate should still admit");
    let shape_mismatch_denied = admit_preview_promotion_parity_comparison(
        &shape_mismatch_preview_execution,
        &shape_mismatch_candidate,
    )
    .expect_err("shape mismatch comparison should reject");
    let read_only_writeback_foundation_denied =
        crate::preview::admit_preview_workflow_foundation_request(
            &active_binding,
            PreviewWorkflowFoundationRequest::deferred_mutation_writeback(),
        )
        .expect_err(
            "read-only preview workflow foundations must deny deferred writeback authority",
        );

    PreviewRejectionEvidence {
        unsupported_preview_family: PreviewCertificationRejection::from_runtime_failure(
            unsupported_preview_family.failure_class(),
            unsupported_preview_family.counters(),
        ),
        invalid_basis: PreviewCertificationRejection::from_runtime_failure(
            invalid_basis.failure_class(),
            invalid_basis.counters(),
        ),
        stale_lifecycle: PreviewCertificationRejection::from_runtime_failure(
            stale_lifecycle.failure_class(),
            stale_lifecycle.counters(),
        ),
        discarded_lifecycle: PreviewCertificationRejection::from_runtime_failure(
            discarded_lifecycle.failure_class(),
            discarded_lifecycle.counters(),
        ),
        preview_live_drift_denied: PreviewCertificationRejection::from_preview_live_failure(
            preview_live_drift_denied.error(),
        ),
        preview_live_broad_fallback_denied:
            PreviewCertificationRejection::from_preview_live_failure(
                preview_live_broad_fallback_denied.error(),
            ),
        read_only_writeback_foundation_denied: PreviewCertificationRejection::from_workflow_failure(
            &read_only_writeback_foundation_denied,
        ),
        promotion_linkage_denied: PreviewCertificationRejection::from_runtime_failure(
            promotion_linkage_denied.failure_class(),
            promotion_linkage_denied.counters(),
        ),
        replay_linkage_denied: PreviewCertificationRejection::from_runtime_failure(
            replay_linkage_denied.failure_class(),
            replay_linkage_denied.counters(),
        ),
        shape_mismatch_denied: PreviewCertificationRejection::from_comparison_failure(
            &shape_mismatch_denied,
        ),
    }
}
