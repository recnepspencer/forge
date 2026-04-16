mod completeness;
mod model;
mod row_catalog;

use crate::harness::certification::{
    CanonicalCertificationRow, ParityAnchor, RejectionCertificationRow,
};
use crate::harness::fixtures::{
    execution_preflights,
    preview_bridge::{
        active_preview_artifacts, declared_preview_session, promoted_preview_artifacts,
        promoted_preview_replay_bundle,
    },
};
use crate::preview::{
    bind_preflight_to_preview_session, PreviewBindingCounters, PreviewBindingFailureClass,
    PreviewBindingIntent, PreviewEvaluationClass, PreviewSessionPlanBinding,
    PreviewSessionQueryContext,
};
use model::{MilestoneFivePointTwoPreviewCertificationArtifact, PreviewCertificationMatrix};
use row_catalog::{
    PreviewCanonicalRowSpec, PreviewRejectionRowSpec, PREVIEW_CANONICAL_ROW_SPECS,
    PREVIEW_REJECTION_ROW_SPECS,
};
#[allow(unused_imports)]
pub(crate) use row_catalog::{
    PREVIEW_REQUIRED_CANONICAL_ROW_NAMES, PREVIEW_REQUIRED_REJECTION_ROW_NAMES,
    PREVIEW_MINIMUM_SPEC_CANONICAL_ROW_NAMES, PREVIEW_MINIMUM_SPEC_REJECTION_ROW_NAMES,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PreviewPerturbationClass {
    ActiveBinding,
    LifecycleExplicitness,
    NoRediscovery,
    InvalidBasis,
    StaleLifecycle,
    RawBranchAliasForbidden,
    PreviewLiveDenied,
    PromotionLinkageDenied,
    ReplayLinkageDenied,
    PromotionEligibilityBoolForbidden,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewLaneEvaluationClass {
    ReadOnly,
    PromotionEligible,
}

impl PreviewLaneEvaluationClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::PromotionEligible => "promotion_eligible",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewLaneLifecycleState {
    Active,
    Admitted,
    Declared,
    Promoted,
    Discarded,
}

impl PreviewLaneLifecycleState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Admitted => "Admitted",
            Self::Declared => "Declared",
            Self::Promoted => "Promoted",
            Self::Discarded => "Discarded",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewFailureClass {
    InvalidPreviewBasis,
    StoreBackedRouteForbidden,
    StaleOrInactivePreviewLifecycle,
    PreviewLiveDeniedInPhaseTwo,
    PromotionLinkageMismatch,
    CompileFail,
}

impl PreviewFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidPreviewBasis => "invalid-preview-basis",
            Self::StoreBackedRouteForbidden => "store-backed-route-forbidden",
            Self::StaleOrInactivePreviewLifecycle => "stale-or-inactive-preview-lifecycle",
            Self::PreviewLiveDeniedInPhaseTwo => "preview-live-denied-in-phase-two",
            Self::PromotionLinkageMismatch => "promotion-linkage-mismatch",
            Self::CompileFail => "compile_fail",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewCertificationLane {
    pub query_digest: String,
    pub result_shape_digest: String,
    pub preview_session_identity: String,
    pub evaluation_class: PreviewLaneEvaluationClass,
    pub lifecycle_state_kind: PreviewLaneLifecycleState,
    pub binding_digest: String,
    pub counters: PreviewBindingCounters,
}

impl PreviewCertificationLane {
    fn from_binding(binding: &PreviewSessionPlanBinding) -> Self {
        Self {
            query_digest: binding
                .basis()
                .binding_tuple()
                .canonical_query_digest()
                .as_str()
                .to_string(),
            result_shape_digest: binding
                .basis()
                .binding_tuple()
                .canonical_result_shape_digest()
                .as_str()
                .to_string(),
            preview_session_identity: binding
                .basis()
                .binding_tuple()
                .preview_session_identity()
                .as_str()
                .to_string(),
            evaluation_class: match binding.basis().binding_tuple().evaluation_class() {
                PreviewEvaluationClass::ReadOnly(_) => PreviewLaneEvaluationClass::ReadOnly,
                PreviewEvaluationClass::PromotionEligible(_) => {
                    PreviewLaneEvaluationClass::PromotionEligible
                }
            },
            lifecycle_state_kind: match binding.basis().binding_tuple().lifecycle_state_kind() {
                forge_runtime_bridge::facade::BridgePreviewLifecycleStateKind::Active => {
                    PreviewLaneLifecycleState::Active
                }
                forge_runtime_bridge::facade::BridgePreviewLifecycleStateKind::Admitted => {
                    PreviewLaneLifecycleState::Admitted
                }
                forge_runtime_bridge::facade::BridgePreviewLifecycleStateKind::Declared => {
                    PreviewLaneLifecycleState::Declared
                }
                forge_runtime_bridge::facade::BridgePreviewLifecycleStateKind::Promoted => {
                    PreviewLaneLifecycleState::Promoted
                }
                forge_runtime_bridge::facade::BridgePreviewLifecycleStateKind::Discarded => {
                    PreviewLaneLifecycleState::Discarded
                }
            },
            binding_digest: binding.basis().binding_tuple().digest().to_string(),
            counters: binding.report().counters().clone(),
        }
    }

    pub fn has_required_outputs(&self) -> bool {
        !self.query_digest.is_empty()
            && !self.result_shape_digest.is_empty()
            && !self.preview_session_identity.is_empty()
            && !self.binding_digest.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewCertificationRejection {
    pub failure_class: PreviewFailureClass,
    pub counters: Option<PreviewBindingCounters>,
    pub compile_fail_case: Option<&'static str>,
}

impl PreviewCertificationRejection {
    fn from_runtime_failure(
        failure_class: &PreviewBindingFailureClass,
        counters: &PreviewBindingCounters,
    ) -> Self {
        Self {
            failure_class: match failure_class {
                PreviewBindingFailureClass::InvalidPreviewBasis => {
                    PreviewFailureClass::InvalidPreviewBasis
                }
                PreviewBindingFailureClass::StoreBackedRouteForbidden => {
                    PreviewFailureClass::StoreBackedRouteForbidden
                }
                PreviewBindingFailureClass::StaleOrInactivePreviewLifecycle => {
                    PreviewFailureClass::StaleOrInactivePreviewLifecycle
                }
                PreviewBindingFailureClass::PreviewLiveDeniedInPhaseTwo => {
                    PreviewFailureClass::PreviewLiveDeniedInPhaseTwo
                }
                PreviewBindingFailureClass::PromotionLinkageMismatch => {
                    PreviewFailureClass::PromotionLinkageMismatch
                }
                other => panic!("unsupported preview certification runtime failure: {other:?}"),
            },
            counters: Some(counters.clone()),
            compile_fail_case: None,
        }
    }

    fn compile_fail(case: &'static str) -> Self {
        Self {
            failure_class: PreviewFailureClass::CompileFail,
            counters: None,
            compile_fail_case: Some(case),
        }
    }

    pub fn has_required_outputs(&self) -> bool {
        self.counters.is_some() || self.compile_fail_case.is_some()
    }
}

pub struct MilestoneFivePointTwoPreviewCertificationAdapter;

impl MilestoneFivePointTwoPreviewCertificationAdapter {
    pub fn preview_session_basis_and_promotion_parity_test() -> PreviewCertificationMatrix {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-certification");
        let active_binding = bind_preflight_to_preview_session(
            preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("active preview certification binding should succeed");
        let parity_binding = bind_preflight_to_preview_session(
            execution_preflights::replay_runtime_preflight(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("parity preview certification binding should succeed");
        let promotable_binding = bind_preflight_to_preview_session(
            execution_preflights::direct_runtime_preflight(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        )
        .expect("promotion-eligible binding should succeed");

        let (_invalid_runtime, _invalid_active, foreign_execution_record) =
            active_preview_artifacts("preview-certification-invalid-basis");
        let invalid_basis = bind_preflight_to_preview_session(
            execution_preflights::direct_runtime_preflight(),
            PreviewSessionQueryContext::active(
                &active,
                &foreign_execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect_err("foreign execution record should reject as invalid basis");
        let (_declared_runtime, declared) =
            declared_preview_session("preview-certification-declared");
        let stale_lifecycle = bind_preflight_to_preview_session(
            execution_preflights::direct_runtime_preflight(),
            PreviewSessionQueryContext::declared(&declared, PreviewEvaluationClass::read_only()),
        )
        .expect_err("declared lifecycle should reject");
        let preview_live_denied = bind_preflight_to_preview_session(
            execution_preflights::direct_runtime_preflight(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            )
            .with_binding_intent(PreviewBindingIntent::preview_with_live_lane()),
        )
        .expect_err("preview live request should reject");
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

        let active_lane = PreviewCertificationLane::from_binding(&active_binding);
        let parity_lane = PreviewCertificationLane::from_binding(&parity_binding);
        let promotable_lane = PreviewCertificationLane::from_binding(&promotable_binding);

        PreviewCertificationMatrix {
            suite_name: "Preview Session Basis And Promotion Parity Test",
            rows: PREVIEW_CANONICAL_ROW_SPECS
                .iter()
                .map(|spec| canonical_row(spec, &active_lane, &parity_lane, &promotable_lane))
                .collect(),
            rejection_rows: PREVIEW_REJECTION_ROW_SPECS
                .iter()
                .map(|spec| {
                    rejection_row(
                        spec,
                        &active_lane,
                        &parity_lane,
                        &invalid_basis,
                        &stale_lifecycle,
                        &preview_live_denied,
                        &promotion_linkage_denied,
                        &replay_linkage_denied,
                    )
                })
                .collect(),
        }
    }

    pub fn preview_session_basis_and_promotion_parity_artifact(
    ) -> MilestoneFivePointTwoPreviewCertificationArtifact {
        Self::preview_session_basis_and_promotion_parity_test()
            .into_milestone_five_point_two_artifact()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        row_catalog::{
            PREVIEW_CANONICAL_ROW_SPECS, PREVIEW_MINIMUM_SPEC_CANONICAL_ROW_NAMES,
            PREVIEW_MINIMUM_SPEC_REJECTION_ROW_NAMES, PREVIEW_REJECTION_ROW_SPECS,
            PREVIEW_REQUIRED_CANONICAL_ROW_NAMES, PREVIEW_REQUIRED_REJECTION_ROW_NAMES,
        },
        MilestoneFivePointTwoPreviewCertificationAdapter, PreviewFailureClass,
        PreviewLaneEvaluationClass,
    };
    use crate::harness::certification::{
        milestone_five_point_two_requirements, unmet_required_rows,
    };

    #[test]
    fn preview_certification_adapter_emits_named_matrix() {
        let matrix =
            MilestoneFivePointTwoPreviewCertificationAdapter::preview_session_basis_and_promotion_parity_test();

        assert_eq!(
            matrix.suite_name,
            "Preview Session Basis And Promotion Parity Test"
        );
        for spec in PREVIEW_CANONICAL_ROW_SPECS {
            assert!(matrix.rows.iter().any(|row| row.row_name == spec.row_name));
        }
        for spec in PREVIEW_REJECTION_ROW_SPECS {
            assert!(matrix
                .rejection_rows
                .iter()
                .any(|row| row.row_name == spec.row_name));
        }
    }

    #[test]
    fn preview_certification_matrix_meets_required_rows() {
        let matrix =
            MilestoneFivePointTwoPreviewCertificationAdapter::preview_session_basis_and_promotion_parity_test();
        let requirements = milestone_five_point_two_requirements();
        let implemented_missing = unmet_required_rows(
            &matrix,
            PREVIEW_REQUIRED_CANONICAL_ROW_NAMES,
            PREVIEW_REQUIRED_REJECTION_ROW_NAMES,
        );

        assert!(
            implemented_missing.is_empty(),
            "missing implemented preview rows: {implemented_missing:?}"
        );
        let spec_missing = unmet_required_rows(
            &matrix,
            requirements.required_canonical_rows,
            requirements.required_rejection_rows,
        );
        assert!(
            !spec_missing.is_empty(),
            "preview certification should remain honest about unimplemented spec rows"
        );
        for row_name in PREVIEW_MINIMUM_SPEC_CANONICAL_ROW_NAMES {
            if !PREVIEW_REQUIRED_CANONICAL_ROW_NAMES.contains(row_name) {
                assert!(spec_missing.contains(row_name));
            }
        }
        for row_name in PREVIEW_MINIMUM_SPEC_REJECTION_ROW_NAMES {
            if !PREVIEW_REQUIRED_REJECTION_ROW_NAMES.contains(row_name) {
                assert!(spec_missing.contains(row_name));
            }
        }
        assert!(matrix
            .rows
            .iter()
            .all(|row| row.control_lane.has_required_outputs()));
        assert!(matrix
            .rejection_rows
            .iter()
            .all(|row| row.hostile_lane.has_required_outputs()));
        assert!(matrix.rows.iter().all(|row| row
            .control_lane
            .counters
            .preview_lifecycle_rediscovery_count()
            == 0));
        assert!(matrix.rows.iter().all(|row| row
            .control_lane
            .counters
            .preview_executor_rediscovery_count()
            == 0));
        for spec in PREVIEW_CANONICAL_ROW_SPECS {
            let row = matrix
                .rows
                .iter()
                .find(|row| row.row_name == spec.row_name)
                .unwrap_or_else(|| panic!("missing preview canonical row {}", spec.row_name));
            assert_eq!(row.perturbation_class, spec.perturbation_class);
            assert_eq!(row.hostile_expectation, spec.hostile_expectation);
            if let Some(hostile_eval) = spec.hostile_evaluation_class {
                assert_eq!(row.hostile_lane.evaluation_class, hostile_eval);
            }
            if spec.hostile_expectation
                == crate::harness::certification::HostileExpectation::DistinctFromControl
            {
                assert_ne!(
                    row.control_lane.binding_digest,
                    row.hostile_lane.binding_digest
                );
                assert_ne!(
                    row.control_lane.evaluation_class,
                    row.hostile_lane.evaluation_class
                );
            } else {
                assert_eq!(
                    row.control_lane.evaluation_class,
                    row.hostile_lane.evaluation_class
                );
            }
            assert_eq!(
                row.control_lane.evaluation_class,
                PreviewLaneEvaluationClass::ReadOnly
            );
        }
        for spec in PREVIEW_REJECTION_ROW_SPECS {
            let row = matrix
                .rejection_rows
                .iter()
                .find(|row| row.row_name == spec.row_name)
                .unwrap_or_else(|| panic!("missing preview rejection row {}", spec.row_name));
            assert_eq!(row.perturbation_class, spec.perturbation_class);
            assert_eq!(row.hostile_lane.failure_class, spec.failure_class);
            assert_eq!(row.hostile_lane.compile_fail_case, spec.compile_fail_case);
            if spec.failure_class == PreviewFailureClass::CompileFail {
                assert!(row.hostile_lane.counters.is_none());
            } else {
                assert!(row.hostile_lane.counters.is_some());
            }
        }
    }

    #[test]
    fn preview_certification_artifact_reports_offline_ready_completeness() {
        let artifact = MilestoneFivePointTwoPreviewCertificationAdapter::
            preview_session_basis_and_promotion_parity_artifact();

        assert_eq!(
            artifact.suite_name,
            "Preview Session Basis And Promotion Parity Test"
        );
        assert!(!artifact.certification_bundle_digest.is_empty());
        assert!(!artifact.coverage_matrix_digest.is_empty());
        assert!(
            !artifact
                .bundle_completeness_report
                .covers_full_milestone_five_point_two_spec_matrix
        );
        assert!(
            artifact
                .bundle_completeness_report
                .covers_all_currently_implemented_normative_scenarios
        );
        assert!(!artifact.bundle_completeness_report.offline_analysis_ready);
        assert_eq!(
            artifact
                .bundle_completeness_report
                .zero_rediscovery_lane_count,
            artifact.bundle_completeness_report.supported_lane_count
        );
        assert!(
            artifact.counter_snapshot.preview_invalid_basis_denial_count() > 0,
            "artifact counter snapshot should retain hostile invalid-basis denials"
        );
        assert!(
            artifact.counter_snapshot.preview_invalid_lifecycle_denial_count() > 0,
            "artifact counter snapshot should retain hostile stale-lifecycle denials"
        );
    }
}

fn canonical_row(
    spec: &PreviewCanonicalRowSpec,
    active_lane: &PreviewCertificationLane,
    parity_lane: &PreviewCertificationLane,
    promotable_lane: &PreviewCertificationLane,
) -> CanonicalCertificationRow<PreviewPerturbationClass, PreviewCertificationLane> {
    CanonicalCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        hostile_expectation: spec.hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane: active_lane.clone(),
        hostile_lane: match spec.hostile_lane_selector {
            row_catalog::PreviewLaneSelector::Parity => parity_lane.clone(),
            row_catalog::PreviewLaneSelector::PromotionEligible => promotable_lane.clone(),
        },
        parity_lane: active_lane.clone(),
    }
}

fn rejection_row(
    spec: &PreviewRejectionRowSpec,
    active_lane: &PreviewCertificationLane,
    parity_lane: &PreviewCertificationLane,
    invalid_basis: &crate::preview::PreviewBindingError,
    stale_lifecycle: &crate::preview::PreviewBindingError,
    preview_live_denied: &crate::preview::PreviewBindingError,
    promotion_linkage_denied: &crate::preview::PreviewBindingError,
    replay_linkage_denied: &crate::preview::PreviewBindingError,
) -> RejectionCertificationRow<
    PreviewPerturbationClass,
    PreviewCertificationLane,
    PreviewCertificationRejection,
> {
    let hostile_lane = match spec.runtime_failure_selector {
        Some(row_catalog::PreviewRuntimeFailureSelector::InvalidBasis) => {
            PreviewCertificationRejection::from_runtime_failure(
                invalid_basis.failure_class(),
                invalid_basis.counters(),
            )
        }
        Some(row_catalog::PreviewRuntimeFailureSelector::StaleLifecycle) => {
            PreviewCertificationRejection::from_runtime_failure(
                stale_lifecycle.failure_class(),
                stale_lifecycle.counters(),
            )
        }
        Some(row_catalog::PreviewRuntimeFailureSelector::PreviewLiveDenied) => {
            PreviewCertificationRejection::from_runtime_failure(
                preview_live_denied.failure_class(),
                preview_live_denied.counters(),
            )
        }
        Some(row_catalog::PreviewRuntimeFailureSelector::PromotionLinkageDenied) => {
            PreviewCertificationRejection::from_runtime_failure(
                promotion_linkage_denied.failure_class(),
                promotion_linkage_denied.counters(),
            )
        }
        Some(row_catalog::PreviewRuntimeFailureSelector::ReplayLinkageDenied) => {
            PreviewCertificationRejection::from_runtime_failure(
                replay_linkage_denied.failure_class(),
                replay_linkage_denied.counters(),
            )
        }
        None => PreviewCertificationRejection::compile_fail(
            spec.compile_fail_case
                .expect("compile-fail preview rejection rows must declare a case"),
        ),
    };

    RejectionCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        control_lane: active_lane.clone(),
        hostile_lane,
        parity_lane: parity_lane.clone(),
    }
}
