mod model;
mod row_catalog;
mod tests;

use crate::correspondence::{
    resolve_correspondence_evidence, CorrespondenceEvaluationRequest,
    StructuralCandidateDiscoveryPlan, StructuralCandidateOrderingContract,
};
use crate::correspondence_history::{
    compose_correspondence_historical_envelope, compose_historical_admission_denied_envelope,
    compose_historical_path_denied_envelope, CorrespondenceHistoricalEnvelope,
};
use crate::facade::{
    admit_historical_evaluation_path, build_correspondence_historical_parity_bundle,
    plan_validated_bundle, preflight_execution_basis, resolve_historical_materialization_path,
    HistoricalCapabilityDescriptor, HistoricalEvaluationError, HistoricalEvaluationRequest,
    HistoricalMaterializationDescriptor, HistoricalPathResolved,
    HistoricalPathReuseDescriptor, RequestedHistoricalPathClass, ResolvedHistoricalPathClass,
};
use crate::harness::certification::{
    milestone_five_point_four_requirements, unmet_required_assertion_classes,
    unmet_required_rows, CertificationMatrix, ParityAnchor, RequiredAssertionClass,
};

pub(crate) use model::{
    CorrespondenceHistoryBundleCompletenessReport, CorrespondenceHistoryCertificationLane,
    CorrespondenceHistoryCertificationMatrix, CorrespondenceHistoryCertificationRejection,
    CorrespondenceHistoryFailureClass, CorrespondenceHistoryPerturbationClass,
    MilestoneFivePointFourCorrespondenceHistoryCertificationArtifact,
};
pub(crate) use row_catalog::{
    CORRESPONDENCE_HISTORY_CANONICAL_ROW_SPECS, CORRESPONDENCE_HISTORY_REJECTION_ROW_SPECS,
    CORRESPONDENCE_HISTORY_REQUIRED_CANONICAL_ROW_NAMES,
    CORRESPONDENCE_HISTORY_REQUIRED_REJECTION_ROW_NAMES,
};

pub struct MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter;

impl MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter {
    pub fn structural_correspondence_and_historical_materialization_path_test(
    ) -> CorrespondenceHistoryCertificationMatrix {
        let lineage_lane = lineage_authoritative_lane();
        let structural_lane = structural_unique_replay_lane();
        let disagreement_lane = disagreement_lane();
        let ambiguity_lane = ambiguity_lane();
        let retained_lane = retained_lane();
        let replay_lane = replay_lane();
        let reconstruction_lane = reconstruction_lane();
        let drift_lane = prediction_drift_lane();

        CertificationMatrix {
            suite_name: "Structural Correspondence And Historical Materialization Path Test",
            rows: CORRESPONDENCE_HISTORY_CANONICAL_ROW_SPECS
                .iter()
                .map(|spec| {
                    canonical_row(
                        spec,
                        &lineage_lane,
                        &structural_lane,
                        &disagreement_lane,
                        &ambiguity_lane,
                        &retained_lane,
                        &replay_lane,
                        &reconstruction_lane,
                        &drift_lane,
                    )
                })
                .collect(),
            rejection_rows: CORRESPONDENCE_HISTORY_REJECTION_ROW_SPECS
                .iter()
                .map(|spec| rejection_row(spec, &lineage_lane, &structural_lane, &replay_lane))
                .collect(),
        }
    }

    pub fn structural_correspondence_and_historical_materialization_path_artifact(
    ) -> MilestoneFivePointFourCorrespondenceHistoryCertificationArtifact {
        let matrix = Self::structural_correspondence_and_historical_materialization_path_test();
        let requirements = milestone_five_point_four_requirements();
        let all_lanes_emit_required_outputs = matrix.rows.iter().all(|row| {
            row.control_lane.has_required_outputs()
                && row.hostile_lane.has_required_outputs()
                && row.parity_lane.has_required_outputs()
        }) && matrix
            .rejection_rows
            .iter()
            .all(|row| row.hostile_lane.has_required_outputs());
        let zero_rediscovery_lane_count = matrix
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .filter(|lane| lane.has_zero_rediscovery())
            .count();
        let supported_lane_count = matrix.rows.len() * 3;
        let completeness = CorrespondenceHistoryBundleCompletenessReport {
            canonical_row_count: matrix.rows.len(),
            rejection_row_count: matrix.rejection_rows.len(),
            all_lanes_emit_required_outputs,
            zero_rediscovery_lane_count,
            unmet_required_rows: unmet_required_rows(
                &matrix,
                requirements.required_canonical_rows,
                requirements.required_rejection_rows,
            ),
            unmet_required_assertion_classes: unmet_required_assertion_classes(
                &[
                    RequiredAssertionClass::Equality,
                    RequiredAssertionClass::Inequality,
                    RequiredAssertionClass::TypedFailure,
                    RequiredAssertionClass::ZeroResidue,
                ],
                requirements.required_assertion_classes,
            ),
            offline_analysis_ready: all_lanes_emit_required_outputs
                && zero_rediscovery_lane_count == supported_lane_count,
        };

        matrix.into_milestone_five_point_four_artifact(completeness)
    }
}

fn canonical_row(
    spec: &row_catalog::CorrespondenceHistoryCanonicalRowSpec,
    lineage_lane: &CorrespondenceHistoryCertificationLane,
    structural_lane: &CorrespondenceHistoryCertificationLane,
    disagreement_lane: &CorrespondenceHistoryCertificationLane,
    ambiguity_lane: &CorrespondenceHistoryCertificationLane,
    retained_lane: &CorrespondenceHistoryCertificationLane,
    replay_lane: &CorrespondenceHistoryCertificationLane,
    reconstruction_lane: &CorrespondenceHistoryCertificationLane,
    drift_lane: &CorrespondenceHistoryCertificationLane,
) -> model::CorrespondenceHistoryCertificationRow {
    let (control_lane, hostile_lane, parity_lane) = match spec.row_name {
        "lineage-correspondence-authoritative" => (
            lineage_lane.clone(),
            lineage_lane.clone(),
            lineage_lane.clone(),
        ),
        "structural-correspondence-advisory" => (
            structural_lane.clone(),
            lineage_lane.clone(),
            structural_lane.clone(),
        ),
        "lineage-structural-disagreement-explicit" => (
            disagreement_lane.clone(),
            disagreement_lane.clone(),
            disagreement_lane.clone(),
        ),
        "structural-ambiguity-explicit" => (
            ambiguity_lane.clone(),
            ambiguity_lane.clone(),
            ambiguity_lane.clone(),
        ),
        "historical-retained-snapshot-path" => (
            retained_lane.clone(),
            retained_lane.clone(),
            retained_lane.clone(),
        ),
        "historical-delta-replay-path" => (
            replay_lane.clone(),
            retained_lane.clone(),
            replay_lane.clone(),
        ),
        "historical-full-reconstruction-path" => (
            reconstruction_lane.clone(),
            reconstruction_lane.clone(),
            reconstruction_lane.clone(),
        ),
        "prediction-drift-explicit" => (drift_lane.clone(), drift_lane.clone(), drift_lane.clone()),
        other => panic!("unknown 5.4 canonical row {other}"),
    };

    model::CorrespondenceHistoryCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        hostile_expectation: spec.hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}

fn rejection_row(
    spec: &row_catalog::CorrespondenceHistoryRejectionRowSpec,
    lineage_lane: &CorrespondenceHistoryCertificationLane,
    structural_lane: &CorrespondenceHistoryCertificationLane,
    replay_lane: &CorrespondenceHistoryCertificationLane,
) -> model::CorrespondenceHistoryRejectionRow {
    let (control_lane, hostile_lane, parity_lane) = match spec.row_name {
        "structural-as-authoritative-forbidden" => (
            lineage_lane.clone(),
            compile_fail_rejection(spec),
            lineage_lane.clone(),
        ),
        "ambiguous-correspondence-not-collapsed" => (
            structural_lane.clone(),
            compile_fail_rejection(spec),
            structural_lane.clone(),
        ),
        "unsupported-correspondence-family" => (
            structural_lane.clone(),
            unsupported_correspondence_family_rejection(),
            structural_lane.clone(),
        ),
        "unsupported-historical-materialization-path" => (
            replay_lane.clone(),
            unsupported_historical_materialization_rejection(),
            replay_lane.clone(),
        ),
        "hidden-materialization-path-substitution-forbidden" => (
            replay_lane.clone(),
            hidden_materialization_substitution_rejection(),
            replay_lane.clone(),
        ),
        "broad-candidate-scan-success-forbidden" => (
            structural_lane.clone(),
            broad_candidate_scan_rejection(),
            structural_lane.clone(),
        ),
        "no-executor-path-mutation-after-planning" => (
            replay_lane.clone(),
            executor_path_mutation_rejection(),
            replay_lane.clone(),
        ),
        "host-cache-history-authority-forbidden" => (
            replay_lane.clone(),
            host_cache_history_authority_rejection(),
            replay_lane.clone(),
        ),
        other => panic!("unknown 5.4 rejection row {other}"),
    };

    model::CorrespondenceHistoryRejectionRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}

fn compile_fail_rejection(
    spec: &row_catalog::CorrespondenceHistoryRejectionRowSpec,
) -> CorrespondenceHistoryCertificationRejection {
    CorrespondenceHistoryCertificationRejection {
        failure_class: spec.failure_class,
        failure_digest: format!("compile_fail:{}", spec.row_name),
        counter_snapshot_digest: None,
        compile_fail_case: spec.compile_fail_case,
    }
}

fn unsupported_correspondence_family_rejection() -> CorrespondenceHistoryCertificationRejection {
    let preflight = detail_preflight_bundle();
    let correspondence = resolve_correspondence_evidence(
        CorrespondenceEvaluationRequest::unsupported_structural_family(
            "unsupported_test_family",
            StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
            1,
        ),
    )
    .expect("unsupported structural family should resolve into denial");
    let envelope = compose_correspondence_historical_envelope(
        crate::execution::execute_preflight_bundle(&preflight).expect("execution should succeed"),
        correspondence,
        retained_resolved("basis:a"),
    );
    let parity_bundle = build_correspondence_historical_parity_bundle(
        &envelope,
        Some(preflight.plan().query().validated_query_digest().clone()),
        Some(preflight.basis().proof().digest().clone()),
    )
    .expect("unsupported correspondence denial bundle should build");
    CorrespondenceHistoryCertificationRejection {
        failure_class: CorrespondenceHistoryFailureClass::CorrespondenceDenied,
        failure_digest: parity_bundle
            .failure_digest()
            .expect("unsupported correspondence denial should emit failure digest")
            .as_str()
            .to_string(),
        counter_snapshot_digest: Some(parity_bundle.counter_snapshot_digest().as_str().to_string()),
        compile_fail_case: None,
    }
}

fn broad_candidate_scan_rejection() -> CorrespondenceHistoryCertificationRejection {
    let error = crate::correspondence::CorrespondenceEvaluationError::BroadStructuralScanRequired;
    CorrespondenceHistoryCertificationRejection {
        failure_class: CorrespondenceHistoryFailureClass::CorrespondenceDenied,
        failure_digest: format!(
            "correspondence:{:?}:{}",
            error.failure_class(),
            error.reason()
        ),
        counter_snapshot_digest: Some(
            build_correspondence_historical_parity_bundle(
                &correspondence_denied_envelope(),
                Some(
                    detail_preflight_bundle()
                        .plan()
                        .query()
                        .validated_query_digest()
                        .clone(),
                ),
                Some(detail_preflight_bundle().basis().proof().digest().clone()),
            )
            .expect("denied correspondence bundle should build")
            .counter_snapshot_digest()
            .as_str()
            .to_string(),
        ),
        compile_fail_case: None,
    }
}

fn unsupported_historical_materialization_rejection() -> CorrespondenceHistoryCertificationRejection
{
    let preflight = detail_preflight_bundle();
    let correspondence =
        resolve_correspondence_evidence(CorrespondenceEvaluationRequest::lineage_only(
            "subject:a",
            "record:a",
            StructuralCandidateDiscoveryPlan::IndexBackedBounded,
            1,
        ))
        .expect("correspondence should resolve");
    let request = HistoricalEvaluationRequest::retained_snapshot(
        "basis:a",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::new(
        "basis:a",
        Some(crate::historical::AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath),
        false,
        false,
        true,
        false,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("admission should succeed");
    let error = HistoricalEvaluationError::UnsupportedBridgeMaterializationPath {
        requested_path_class: RequestedHistoricalPathClass::RequestedRetainedSnapshotPath,
        path_name: "unsupported_test_path",
    };
    let envelope =
        compose_historical_path_denied_envelope(correspondence, admission, error.clone());
    let parity_bundle = build_correspondence_historical_parity_bundle(
        &envelope,
        Some(preflight.plan().query().validated_query_digest().clone()),
        Some(preflight.basis().proof().digest().clone()),
    )
    .expect("unsupported historical path bundle should build");

    CorrespondenceHistoryCertificationRejection {
        failure_class: CorrespondenceHistoryFailureClass::HistoricalPathDenied,
        failure_digest: parity_bundle
            .failure_digest()
            .expect("historical denial should emit failure digest")
            .as_str()
            .to_string(),
        counter_snapshot_digest: Some(parity_bundle.counter_snapshot_digest().as_str().to_string()),
        compile_fail_case: None,
    }
}

fn hidden_materialization_substitution_rejection() -> CorrespondenceHistoryCertificationRejection {
    let preflight = detail_preflight_bundle();
    let correspondence =
        resolve_correspondence_evidence(CorrespondenceEvaluationRequest::lineage_only(
            "subject:a",
            "record:a",
            StructuralCandidateDiscoveryPlan::IndexBackedBounded,
            1,
        ))
        .expect("correspondence should resolve");
    let request = HistoricalEvaluationRequest::delta_replay(
        "basis:replay",
        4,
        8,
        HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::new(
        "basis:replay",
        Some(crate::historical::AdmittedHistoricalPathClass::AdmittedDeltaReplayPath),
        true,
        false,
        false,
        true,
        HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("admission should succeed");
    let error = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::new(
            "basis:replay",
            ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath,
        ),
    )
    .expect_err("path mutation should fail");
    let envelope = compose_historical_path_denied_envelope(correspondence, admission, error);
    let parity_bundle = build_correspondence_historical_parity_bundle(
        &envelope,
        Some(preflight.plan().query().validated_query_digest().clone()),
        Some(preflight.basis().proof().digest().clone()),
    )
    .expect("substitution denied bundle should build");

    CorrespondenceHistoryCertificationRejection {
        failure_class: CorrespondenceHistoryFailureClass::HistoricalPathDenied,
        failure_digest: parity_bundle
            .failure_digest()
            .expect("substitution denied should emit failure digest")
            .as_str()
            .to_string(),
        counter_snapshot_digest: Some(parity_bundle.counter_snapshot_digest().as_str().to_string()),
        compile_fail_case: None,
    }
}

fn lineage_authoritative_lane() -> CorrespondenceHistoryCertificationLane {
    lane_from_supported_envelope(lineage_authoritative_envelope())
}

fn structural_unique_replay_lane() -> CorrespondenceHistoryCertificationLane {
    lane_from_supported_envelope(structural_unique_replay_envelope())
}

fn disagreement_lane() -> CorrespondenceHistoryCertificationLane {
    lane_from_supported_envelope(disagreement_envelope())
}

fn ambiguity_lane() -> CorrespondenceHistoryCertificationLane {
    lane_from_supported_envelope(ambiguity_envelope())
}

fn retained_lane() -> CorrespondenceHistoryCertificationLane {
    lane_from_supported_envelope(retained_path_envelope())
}

fn replay_lane() -> CorrespondenceHistoryCertificationLane {
    lane_from_supported_envelope(replay_path_envelope())
}

fn reconstruction_lane() -> CorrespondenceHistoryCertificationLane {
    lane_from_supported_envelope(reconstruction_path_envelope())
}

fn prediction_drift_lane() -> CorrespondenceHistoryCertificationLane {
    let execution = crate::execution::execute_preflight_bundle(&detail_preflight_bundle())
        .expect("execution should succeed");
    let correspondence =
        resolve_correspondence_evidence(CorrespondenceEvaluationRequest::lineage_only(
            "subject:drift",
            "record:drift",
            StructuralCandidateDiscoveryPlan::IndexBackedBounded,
            1,
        ))
        .expect("correspondence should resolve");
    let request = HistoricalEvaluationRequest::delta_replay(
        "basis:drift",
        1,
        8,
        HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::new(
        "basis:drift",
        Some(crate::historical::AdmittedHistoricalPathClass::AdmittedDeltaReplayPath),
        true,
        false,
        false,
        true,
        HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("admission should succeed");
    let historical = resolve_historical_materialization_path(
        admission,
        HistoricalMaterializationDescriptor::new(
            "basis:drift",
            ResolvedHistoricalPathClass::ResolvedDeltaReplayPath,
        )
        .with_realized_work(3, 0),
    )
    .expect("historical drift path should resolve");

    lane_from_supported_envelope(compose_correspondence_historical_envelope(
        execution,
        correspondence,
        historical,
    ))
}

fn executor_path_mutation_rejection() -> CorrespondenceHistoryCertificationRejection {
    let preflight = detail_preflight_bundle();
    let correspondence =
        resolve_correspondence_evidence(CorrespondenceEvaluationRequest::lineage_only(
            "subject:a",
            "record:a",
            StructuralCandidateDiscoveryPlan::IndexBackedBounded,
            1,
        ))
        .expect("correspondence should resolve");
    let request = HistoricalEvaluationRequest::retained_snapshot(
        "basis:executor",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::new(
        "basis:executor",
        Some(crate::historical::AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath),
        false,
        false,
        true,
        false,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("admission should succeed");
    let error = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::new(
            "basis:executor",
            ResolvedHistoricalPathClass::ResolvedFullReconstructionPath,
        ),
    )
    .expect_err("executor mutation should fail");
    let envelope = compose_historical_path_denied_envelope(correspondence, admission, error);
    let parity_bundle = build_correspondence_historical_parity_bundle(
        &envelope,
        Some(preflight.plan().query().validated_query_digest().clone()),
        Some(preflight.basis().proof().digest().clone()),
    )
    .expect("executor mutation denial bundle should build");

    CorrespondenceHistoryCertificationRejection {
        failure_class: CorrespondenceHistoryFailureClass::HistoricalPathDenied,
        failure_digest: parity_bundle
            .failure_digest()
            .expect("executor mutation denial should emit failure digest")
            .as_str()
            .to_string(),
        counter_snapshot_digest: Some(parity_bundle.counter_snapshot_digest().as_str().to_string()),
        compile_fail_case: None,
    }
}

fn host_cache_history_authority_rejection() -> CorrespondenceHistoryCertificationRejection {
    let preflight = detail_preflight_bundle();
    let correspondence =
        resolve_correspondence_evidence(CorrespondenceEvaluationRequest::lineage_only(
            "subject:a",
            "record:a",
            StructuralCandidateDiscoveryPlan::IndexBackedBounded,
            1,
        ))
        .expect("correspondence should resolve");
    let request = HistoricalEvaluationRequest::retained_snapshot(
        "basis:host-cache",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let error = HistoricalEvaluationError::UnsupportedHistoricalPathRequest {
        requested_path_class: RequestedHistoricalPathClass::RequestedRetainedSnapshotPath,
        reason: "historical evaluation authority may not be satisfied from host cache state",
    };
    let envelope = compose_historical_admission_denied_envelope(correspondence, request, error);
    let parity_bundle = build_correspondence_historical_parity_bundle(
        &envelope,
        Some(preflight.plan().query().validated_query_digest().clone()),
        Some(preflight.basis().proof().digest().clone()),
    )
    .expect("host cache authority denial bundle should build");

    CorrespondenceHistoryCertificationRejection {
        failure_class: CorrespondenceHistoryFailureClass::HistoricalPathDenied,
        failure_digest: parity_bundle
            .failure_digest()
            .expect("host cache denial should emit failure digest")
            .as_str()
            .to_string(),
        counter_snapshot_digest: Some(parity_bundle.counter_snapshot_digest().as_str().to_string()),
        compile_fail_case: None,
    }
}

fn lane_from_supported_envelope(
    envelope: CorrespondenceHistoricalEnvelope,
) -> CorrespondenceHistoryCertificationLane {
    CorrespondenceHistoryCertificationLane {
        parity_bundle: build_correspondence_historical_parity_bundle(&envelope, None, None)
            .expect("supported envelope parity bundle should build"),
    }
}

fn correspondence_denied_envelope() -> CorrespondenceHistoricalEnvelope {
    let execution = crate::execution::execute_preflight_bundle(&detail_preflight_bundle())
        .expect("execution should succeed");
    let correspondence =
        resolve_correspondence_evidence(CorrespondenceEvaluationRequest::structural_only(
            vec!["record:a".into(), "record:b".into(), "record:c".into()],
            StructuralCandidateDiscoveryPlan::RequiresBroadScanDenied,
            2,
            StructuralCandidateOrderingContract::StableFingerprintOrder,
        ))
        .expect("correspondence should resolve into denial");
    let request = HistoricalEvaluationRequest::retained_snapshot(
        "basis:a",
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::new(
        "basis:a",
        Some(crate::historical::AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath),
        false,
        false,
        true,
        false,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("admission should succeed");
    let resolved = resolve_historical_materialization_path(
        admission,
        HistoricalMaterializationDescriptor::new(
            "basis:a",
            ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath,
        ),
    )
    .expect("resolution should succeed");

    compose_correspondence_historical_envelope(execution, correspondence, resolved)
}

fn lineage_authoritative_envelope() -> CorrespondenceHistoricalEnvelope {
    let execution = crate::execution::execute_preflight_bundle(&detail_preflight_bundle())
        .expect("execution should succeed");
    let correspondence =
        resolve_correspondence_evidence(CorrespondenceEvaluationRequest::lineage_only(
            "subject:a",
            "record:a",
            StructuralCandidateDiscoveryPlan::IndexBackedBounded,
            1,
        ))
        .expect("correspondence should resolve");
    let historical = retained_resolved("basis:a");
    compose_correspondence_historical_envelope(execution, correspondence, historical)
}

fn structural_unique_replay_envelope() -> CorrespondenceHistoricalEnvelope {
    let execution = crate::execution::execute_preflight_bundle(&detail_preflight_bundle())
        .expect("execution should succeed");
    let correspondence =
        resolve_correspondence_evidence(CorrespondenceEvaluationRequest::structural_only(
            vec!["record:structural".into()],
            StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
            2,
            StructuralCandidateOrderingContract::StableFingerprintOrder,
        ))
        .expect("correspondence should resolve");
    let historical = replay_resolved("basis:replay");
    compose_correspondence_historical_envelope(execution, correspondence, historical)
}

fn disagreement_envelope() -> CorrespondenceHistoricalEnvelope {
    let execution = crate::execution::execute_preflight_bundle(&detail_preflight_bundle())
        .expect("execution should succeed");
    let correspondence = resolve_correspondence_evidence(CorrespondenceEvaluationRequest::mixed(
        "subject:a",
        "record:a",
        vec!["record:z".into()],
        StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
        2,
        StructuralCandidateOrderingContract::StableFingerprintThenLineageHintOrder,
    ))
    .expect("correspondence should resolve");
    let historical = retained_resolved("basis:a");
    compose_correspondence_historical_envelope(execution, correspondence, historical)
}

fn ambiguity_envelope() -> CorrespondenceHistoricalEnvelope {
    let execution = crate::execution::execute_preflight_bundle(&collection_preflight_bundle())
        .expect("execution should succeed");
    let correspondence =
        resolve_correspondence_evidence(CorrespondenceEvaluationRequest::structural_only(
            vec!["record:a".into(), "record:b".into()],
            StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
            4,
            StructuralCandidateOrderingContract::StableFingerprintOrder,
        ))
        .expect("correspondence should resolve");
    let historical = replay_resolved("basis:replay");
    compose_correspondence_historical_envelope(execution, correspondence, historical)
}

fn retained_path_envelope() -> CorrespondenceHistoricalEnvelope {
    let execution = crate::execution::execute_preflight_bundle(&detail_preflight_bundle())
        .expect("execution should succeed");
    let correspondence =
        resolve_correspondence_evidence(CorrespondenceEvaluationRequest::lineage_only(
            "subject:a",
            "record:a",
            StructuralCandidateDiscoveryPlan::IndexBackedBounded,
            1,
        ))
        .expect("correspondence should resolve");
    compose_correspondence_historical_envelope(
        execution,
        correspondence,
        retained_resolved("basis:a"),
    )
}

fn replay_path_envelope() -> CorrespondenceHistoricalEnvelope {
    let execution = crate::execution::execute_preflight_bundle(&detail_preflight_bundle())
        .expect("execution should succeed");
    let correspondence =
        resolve_correspondence_evidence(CorrespondenceEvaluationRequest::lineage_only(
            "subject:a",
            "record:a",
            StructuralCandidateDiscoveryPlan::IndexBackedBounded,
            1,
        ))
        .expect("correspondence should resolve");
    compose_correspondence_historical_envelope(
        execution,
        correspondence,
        replay_resolved("basis:replay"),
    )
}

fn reconstruction_path_envelope() -> CorrespondenceHistoricalEnvelope {
    let execution = crate::execution::execute_preflight_bundle(&detail_preflight_bundle())
        .expect("execution should succeed");
    let correspondence =
        resolve_correspondence_evidence(CorrespondenceEvaluationRequest::lineage_only(
            "subject:a",
            "record:a",
            StructuralCandidateDiscoveryPlan::IndexBackedBounded,
            1,
        ))
        .expect("correspondence should resolve");
    compose_correspondence_historical_envelope(
        execution,
        correspondence,
        reconstruction_resolved("basis:reconstruct"),
    )
}

fn retained_resolved(basis: &str) -> HistoricalPathResolved {
    let request = HistoricalEvaluationRequest::retained_snapshot(
        basis.to_string(),
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::new(
        basis.to_string(),
        Some(crate::historical::AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath),
        false,
        false,
        true,
        false,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("admission should succeed");
    resolve_historical_materialization_path(
        admission,
        HistoricalMaterializationDescriptor::new(
            basis.to_string(),
            ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath,
        ),
    )
    .expect("resolution should succeed")
}

fn replay_resolved(basis: &str) -> HistoricalPathResolved {
    let request = HistoricalEvaluationRequest::delta_replay(
        basis.to_string(),
        4,
        8,
        HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::new(
        basis.to_string(),
        Some(crate::historical::AdmittedHistoricalPathClass::AdmittedDeltaReplayPath),
        true,
        false,
        false,
        true,
        HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("admission should succeed");
    resolve_historical_materialization_path(
        admission,
        HistoricalMaterializationDescriptor::new(
            basis.to_string(),
            ResolvedHistoricalPathClass::ResolvedDeltaReplayPath,
        ),
    )
    .expect("resolution should succeed")
}

fn reconstruction_resolved(basis: &str) -> HistoricalPathResolved {
    let request = HistoricalEvaluationRequest::full_reconstruction(
        basis.to_string(),
        4,
        8,
        HistoricalPathReuseDescriptor::no_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::new(
        basis.to_string(),
        Some(crate::historical::AdmittedHistoricalPathClass::AdmittedFullReconstructionPath),
        true,
        false,
        false,
        true,
        HistoricalPathReuseDescriptor::no_reuse(),
    );
    let admission =
        admit_historical_evaluation_path(request, capability).expect("admission should succeed");
    resolve_historical_materialization_path(
        admission,
        HistoricalMaterializationDescriptor::new(
            basis.to_string(),
            ResolvedHistoricalPathClass::ResolvedFullReconstructionPath,
        ),
    )
    .expect("resolution should succeed")
}

fn detail_preflight_bundle() -> crate::basis::ExecutionPreflightBundle {
    let validated = crate::harness::fixtures::validated_bundles::runtime_detail_bundle();
    let request = crate::harness::fixtures::planning_requests::direct_runtime_request(&validated);
    let basis = crate::harness::fixtures::resolved_bases::runtime_basis(&validated, "snapshot-1");
    let plan =
        plan_validated_bundle(&validated, request).expect("detail validated bundle should plan");
    preflight_execution_basis(plan, basis).expect("detail plan should preflight")
}

fn collection_preflight_bundle() -> crate::basis::ExecutionPreflightBundle {
    let validated = crate::harness::fixtures::validated_bundles::ordered_collection_bundle();
    let request = crate::harness::fixtures::planning_requests::direct_runtime_request(&validated);
    let basis = crate::harness::fixtures::resolved_bases::runtime_basis(&validated, "snapshot-1");
    let plan = plan_validated_bundle(&validated, request)
        .expect("collection validated bundle should plan");
    preflight_execution_basis(plan, basis).expect("collection plan should preflight")
}
