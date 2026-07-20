mod completeness;
mod model;
mod row_catalog;

use crate::harness::certification::{
    CanonicalCertificationRow, ParityAnchor, RejectionCertificationRow,
};
use crate::harness::fixtures::{
    execution_preflights,
    preview_bridge::{
        active_preview_artifacts, declared_preview_session, discarded_preview_artifacts,
        promoted_preview_artifacts, promoted_preview_replay_bundle,
    },
};
use crate::preview::{
    admit_authoritative_preview_comparison_candidate, admit_preview_promotion_parity_comparison,
    admit_promotion_eligible_preview_session_plan_binding,
    admit_read_only_preview_session_plan_binding, admit_scoped_preview_live_session_plan,
    admit_scoped_preview_session_plan_binding_from_preview_binding, assess_preview_live_drift,
    bind_preflight_to_preview_session, execute_promotion_eligible_preview_session_plan,
    execute_read_only_preview_session_plan, execute_scoped_preview_live_session_plan,
    PreviewBindingCounters, PreviewBindingError, PreviewBindingFailureClass,
    PreviewComparisonCounters, PreviewComparisonError, PreviewComparisonFailureClass,
    PreviewEvaluationClass, PreviewExecutionCounters, PreviewExecutionEnvelope,
    PreviewLiveCounters, PreviewLiveDriftOutcome, PreviewLiveError, PreviewLiveFailureClass,
    PreviewSessionQueryContext, PreviewWorkflowFoundationRequest,
};
use model::{MilestoneFivePointTwoPreviewCertificationArtifact, PreviewCertificationMatrix};
use row_catalog::{
    PreviewCanonicalRowSpec, PreviewRejectionRowSpec, PREVIEW_CANONICAL_ROW_SPECS,
    PREVIEW_REJECTION_ROW_SPECS,
};
pub(crate) use row_catalog::{
    PREVIEW_REQUIRED_CANONICAL_ROW_NAMES, PREVIEW_REQUIRED_REJECTION_ROW_NAMES,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PreviewPerturbationClass {
    ActiveBinding,
    LifecycleExplicitness,
    NoRediscovery,
    PreviewLiveAdmission,
    PreviewLiveDrift,
    InvalidBasis,
    StaleLifecycle,
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
    UnsupportedPreviewFamily,
    InvalidPreviewBasis,
    StoreBackedRouteForbidden,
    StaleOrInactivePreviewLifecycle,
    PreviewLiveDriftDenied,
    PreviewLiveBroadFallbackForbidden,
    WorkflowFoundationAuthorityDenied,
    PromotionLinkageMismatch,
    PreviewShapeMismatchDenied,
}

impl PreviewFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnsupportedPreviewFamily => "unsupported-preview-family",
            Self::InvalidPreviewBasis => "invalid-preview-basis",
            Self::StoreBackedRouteForbidden => "store-backed-route-forbidden",
            Self::StaleOrInactivePreviewLifecycle => "stale-or-inactive-preview-lifecycle",
            Self::PreviewLiveDriftDenied => "preview-live-drift-denied",
            Self::PreviewLiveBroadFallbackForbidden => "preview-live-broad-fallback-forbidden",
            Self::WorkflowFoundationAuthorityDenied => "workflow-foundation-authority-denied",
            Self::PromotionLinkageMismatch => "promotion-linkage-mismatch",
            Self::PreviewShapeMismatchDenied => "preview-shape-mismatch-denied",
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
    pub preview_execution_digest: String,
    pub comparison_eligibility_digest: String,
    pub workflow_foundation_digest: String,
    pub promotion_parity_digest: Option<String>,
    pub preview_live_digest: Option<String>,
    pub preview_live_subscription_digest: Option<String>,
    pub preview_live_family: Option<String>,
    pub counters: PreviewBindingCounters,
    pub execution_counters: PreviewExecutionCounters,
    pub comparison_counters: Option<PreviewComparisonCounters>,
    pub preview_live_counters: Option<PreviewLiveCounters>,
}

impl PreviewCertificationLane {
    fn from_execution(binding: &PreviewExecutionEnvelope) -> Self {
        let binding_tuple = binding.binding().basis().binding_tuple();
        Self {
            query_digest: binding
                .binding()
                .basis()
                .binding_tuple()
                .canonical_query_digest()
                .as_str()
                .to_string(),
            result_shape_digest: binding
                .binding()
                .basis()
                .binding_tuple()
                .canonical_result_shape_digest()
                .as_str()
                .to_string(),
            preview_session_identity: binding
                .binding()
                .basis()
                .binding_tuple()
                .preview_session_identity()
                .bridge_admission_evidence()
                .terminal_projection_for_reporting()
                .to_string(),
            evaluation_class: match binding_tuple.evaluation_class() {
                PreviewEvaluationClass::ReadOnly(_) => PreviewLaneEvaluationClass::ReadOnly,
                PreviewEvaluationClass::PromotionEligible(_) => {
                    PreviewLaneEvaluationClass::PromotionEligible
                }
            },
            lifecycle_state_kind: match binding_tuple.lifecycle_state_kind() {
                worth_runtime_bridge::facade::BridgePreviewLifecycleStateKind::Active => {
                    PreviewLaneLifecycleState::Active
                }
                worth_runtime_bridge::facade::BridgePreviewLifecycleStateKind::Admitted => {
                    PreviewLaneLifecycleState::Admitted
                }
                worth_runtime_bridge::facade::BridgePreviewLifecycleStateKind::Declared => {
                    PreviewLaneLifecycleState::Declared
                }
                worth_runtime_bridge::facade::BridgePreviewLifecycleStateKind::Promoted => {
                    PreviewLaneLifecycleState::Promoted
                }
                worth_runtime_bridge::facade::BridgePreviewLifecycleStateKind::Discarded => {
                    PreviewLaneLifecycleState::Discarded
                }
            },
            binding_digest: binding_tuple.digest().to_string(),
            preview_execution_digest: binding.report().preview_execution_digest().to_string(),
            comparison_eligibility_digest: binding
                .report()
                .comparison_eligibility_digest()
                .to_string(),
            workflow_foundation_digest: binding.report().workflow_foundation_digest().to_string(),
            promotion_parity_digest: None,
            preview_live_digest: None,
            preview_live_subscription_digest: None,
            preview_live_family: None,
            counters: binding.counters().binding_counters().clone(),
            execution_counters: binding.counters().clone(),
            comparison_counters: None,
            preview_live_counters: None,
        }
    }

    fn with_promotion_parity(
        mut self,
        admission: &crate::preview::PromotionParityPreviewComparisonAdmission,
    ) -> Self {
        self.promotion_parity_digest = Some(admission.as_preview_comparison().digest().to_string());
        self.comparison_counters = Some(admission.as_preview_comparison().counters().clone());
        self
    }

    fn with_preview_live(
        mut self,
        preview_live: &crate::preview::PreviewLiveExecutionEnvelope,
    ) -> Self {
        let admitted = preview_live.preview_live();
        self.preview_live_digest = Some(admitted.report().digest().to_string());
        self.preview_live_subscription_digest =
            Some(admitted.report().live_subscription_digest().to_string());
        self.preview_live_family = Some(admitted.report().live_family().to_string());
        self.preview_live_counters = Some(preview_live.counters().clone());
        self
    }

    fn with_preview_live_rebind(
        mut self,
        rebound_execution: &crate::preview::PreviewLiveExecutionEnvelope,
        rebind: &crate::preview::PreviewLiveRebindArtifact,
    ) -> Self {
        self = self.with_preview_live(rebound_execution);
        let mut counters = self.preview_live_counters.take().unwrap_or_default();
        counters.absorb(rebind.counters());
        self.preview_live_counters = Some(counters);
        self
    }

    pub fn has_required_outputs(&self) -> bool {
        !self.query_digest.is_empty()
            && !self.result_shape_digest.is_empty()
            && !self.preview_session_identity.is_empty()
            && !self.binding_digest.is_empty()
            && !self.preview_execution_digest.is_empty()
            && !self.comparison_eligibility_digest.is_empty()
            && !self.workflow_foundation_digest.is_empty()
            && match (
                self.preview_live_digest.as_ref(),
                self.preview_live_subscription_digest.as_ref(),
                self.preview_live_family.as_ref(),
                self.preview_live_counters.as_ref(),
            ) {
                (None, None, None, None) => true,
                (Some(digest), Some(subscription), Some(family), Some(_)) => {
                    !digest.is_empty() && !subscription.is_empty() && !family.is_empty()
                }
                _ => false,
            }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewCertificationRejection {
    pub failure_class: PreviewFailureClass,
    pub counters: Option<PreviewBindingCounters>,
    pub execution_counters: Option<PreviewExecutionCounters>,
    pub comparison_counters: Option<PreviewComparisonCounters>,
    pub preview_live_counters: Option<PreviewLiveCounters>,
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
                PreviewBindingFailureClass::UnsupportedPreviewQueryFamily => {
                    PreviewFailureClass::UnsupportedPreviewFamily
                }
                PreviewBindingFailureClass::StoreBackedRouteForbidden => {
                    PreviewFailureClass::StoreBackedRouteForbidden
                }
                PreviewBindingFailureClass::StaleOrInactivePreviewLifecycle => {
                    PreviewFailureClass::StaleOrInactivePreviewLifecycle
                }
                PreviewBindingFailureClass::PromotionLinkageMismatch => {
                    PreviewFailureClass::PromotionLinkageMismatch
                }
                other => panic!("unsupported preview certification runtime failure: {other:?}"),
            },
            counters: Some(counters.clone()),
            execution_counters: None,
            comparison_counters: None,
            preview_live_counters: None,
        }
    }

    fn from_comparison_failure(error: &PreviewComparisonError) -> Self {
        Self {
            failure_class: match error.failure_class() {
                PreviewComparisonFailureClass::QueryDigestMismatch
                | PreviewComparisonFailureClass::ResultShapeMismatch
                | PreviewComparisonFailureClass::ResultFamilyMismatch
                | PreviewComparisonFailureClass::OrderingBasisMismatch
                | PreviewComparisonFailureClass::MaterializationBoundaryMismatch => {
                    PreviewFailureClass::PreviewShapeMismatchDenied
                }
                other => panic!("unsupported preview certification comparison failure: {other:?}"),
            },
            counters: None,
            execution_counters: None,
            comparison_counters: Some(error.counters().clone()),
            preview_live_counters: None,
        }
    }

    fn from_workflow_failure(error: &crate::preview::PreviewWorkflowFoundationError) -> Self {
        Self {
            failure_class: match error.failure_class() {
                crate::preview::PreviewWorkflowFoundationFailureClass::ReadOnlyPreviewWritebackFoundationForbidden => {
                    PreviewFailureClass::WorkflowFoundationAuthorityDenied
                }
                crate::preview::PreviewWorkflowFoundationFailureClass::OutOfScopeWorkflowFoundationRequest => {
                    panic!("out-of-scope workflow foundation denial is no longer expected in preview certification")
                }
            },
            counters: None,
            execution_counters: Some(error.counters().clone()),
            comparison_counters: None,
            preview_live_counters: None,
        }
    }

    fn from_preview_live_failure(error: &PreviewLiveError) -> Self {
        Self {
            failure_class: match error.failure_class() {
                PreviewLiveFailureClass::PreviewLiveLifecycleDrifted => {
                    PreviewFailureClass::PreviewLiveDriftDenied
                }
                PreviewLiveFailureClass::PreviewLiveBroadFallbackForbidden => {
                    PreviewFailureClass::PreviewLiveBroadFallbackForbidden
                }
                other => panic!("unsupported preview-live certification failure: {other:?}"),
            },
            counters: None,
            execution_counters: None,
            comparison_counters: None,
            preview_live_counters: Some(error.counters().clone()),
        }
    }

    pub fn has_required_outputs(&self) -> bool {
        self.counters.is_some()
            || self.execution_counters.is_some()
            || self.comparison_counters.is_some()
            || self.preview_live_counters.is_some()
    }
}

pub struct MilestoneFivePointTwoPreviewCertificationAdapter;

impl MilestoneFivePointTwoPreviewCertificationAdapter {
    pub fn preview_session_basis_and_promotion_parity_test() -> PreviewCertificationMatrix {
        let preflight = execution_preflights::direct_runtime_preflight();
        let parity_preflight = execution_preflights::replay_runtime_preflight();
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
        let active_execution = execute_read_only_preview_session_plan(
            &admit_read_only_preview_session_plan_binding(active_binding.clone())
                .expect("active read-only binding should admit"),
        )
        .expect("active preview execution should succeed");
        let parity_binding = bind_preflight_to_preview_session(
            parity_preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("parity preview certification binding should succeed");
        let parity_execution = execute_read_only_preview_session_plan(
            &admit_read_only_preview_session_plan_binding(parity_binding.clone())
                .expect("parity read-only binding should admit"),
        )
        .expect("parity preview execution should succeed");
        let promotable_binding = bind_preflight_to_preview_session(
            preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        )
        .expect("promotion-eligible binding should succeed");
        let promotable_execution = execute_promotion_eligible_preview_session_plan(
            &admit_promotion_eligible_preview_session_plan_binding(promotable_binding.clone())
                .expect("promotion-eligible binding should admit"),
        )
        .expect("promotion-eligible preview execution should succeed");
        let parity_promotable_binding = bind_preflight_to_preview_session(
            parity_preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        )
        .expect("parity promotion-eligible binding should succeed");
        let parity_promotable_execution = execute_promotion_eligible_preview_session_plan(
            &admit_promotion_eligible_preview_session_plan_binding(
                parity_promotable_binding.clone(),
            )
            .expect("parity promotion-eligible binding should admit"),
        )
        .expect("parity promotion-eligible preview execution should succeed");
        let preview_live_binding = admit_scoped_preview_live_session_plan(
            admit_scoped_preview_session_plan_binding_from_preview_binding(
                promotable_binding.clone(),
            )
            .expect("preview-live should derive scoped preview binding"),
            crate::live::promote_preflight_bundle_to_live(&preflight)
                .expect("preview-live should reuse admitted detail live proof"),
        )
        .expect("preview-live admission should succeed");
        let preview_live = execute_scoped_preview_live_session_plan(&preview_live_binding)
            .expect("preview-live execution should succeed");
        let parity_preview_live_binding = admit_scoped_preview_live_session_plan(
            admit_scoped_preview_session_plan_binding_from_preview_binding(
                parity_promotable_binding.clone(),
            )
            .expect("parity preview-live should derive scoped preview binding"),
            crate::live::promote_preflight_bundle_to_live(&parity_preflight)
                .expect("parity preview-live should reuse admitted detail live proof"),
        )
        .expect("parity preview-live admission should succeed");
        let parity_preview_live =
            execute_scoped_preview_live_session_plan(&parity_preview_live_binding)
                .expect("parity preview-live execution should succeed");
        let promotion_candidate_execution = crate::execution::execute_preflight_bundle(&preflight)
            .expect("authoritative comparison candidate should execute");
        let promotion_candidate = admit_authoritative_preview_comparison_candidate(
            &preflight,
            &promotion_candidate_execution,
        )
        .expect("authoritative comparison candidate should admit");
        let promotion_parity =
            admit_preview_promotion_parity_comparison(&promotable_execution, &promotion_candidate)
                .expect("promotion parity comparison should admit");

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
        let (_declared_runtime, declared) =
            declared_preview_session("preview-certification-declared");
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
        let shape_mismatch_candidate_preflight =
            execution_preflights::ordered_collection_preflight();
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
        let (_rebind_old_runtime, rebind_old_active, rebind_old_execution_record) =
            active_preview_artifacts("preview-certification-live-rebind-old");
        let rebind_seed_binding = bind_preflight_to_preview_session(
            preflight.clone(),
            PreviewSessionQueryContext::active(
                &rebind_old_active,
                &rebind_old_execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        )
        .expect("rebind seed preview binding should admit");
        let rebind_seed_preview_live = admit_scoped_preview_live_session_plan(
            admit_scoped_preview_session_plan_binding_from_preview_binding(rebind_seed_binding)
                .expect("rebind seed should derive scoped preview binding"),
            crate::live::promote_preflight_bundle_to_live(&preflight)
                .expect("rebind seed should reuse live proof"),
        )
        .expect("rebind seed preview-live should admit");
        let (_rebind_new_runtime, rebind_new_active, rebind_new_execution_record) =
            active_preview_artifacts("preview-certification-live-rebind-new");
        let preview_live_explicit_rebind = match assess_preview_live_drift(
            &rebind_seed_preview_live,
            PreviewSessionQueryContext::active(
                &rebind_new_active,
                &rebind_new_execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        ) {
            PreviewLiveDriftOutcome::ExplicitRebindAvailable(rebind) => rebind,
            other => panic!("preview-live drift should offer explicit rebind, got {other:?}"),
        };
        let preview_live_rebind_preview_execution =
            execute_promotion_eligible_preview_session_plan(
                &admit_promotion_eligible_preview_session_plan_binding(
                    preview_live_explicit_rebind
                        .rebound_preview_live()
                        .scoped_binding()
                        .preview_binding()
                        .clone(),
                )
                .expect("rebound preview binding should admit"),
            )
            .expect("rebound preview execution should succeed");
        let preview_live_rebind_execution = execute_scoped_preview_live_session_plan(
            &admit_scoped_preview_live_session_plan(
                admit_scoped_preview_session_plan_binding_from_preview_binding(
                    preview_live_explicit_rebind
                        .rebound_preview_live()
                        .scoped_binding()
                        .preview_binding()
                        .clone(),
                )
                .expect("rebound preview-live should derive scoped preview binding"),
                preview_live_explicit_rebind
                    .rebound_preview_live()
                    .live_plan()
                    .clone(),
            )
            .expect("rebound preview-live should admit through scoped path"),
        )
        .expect("rebound preview-live execution should succeed");

        let active_lane =
            PreviewCertificationLane::from_execution(active_execution.as_preview_execution());
        let parity_lane =
            PreviewCertificationLane::from_execution(parity_execution.as_preview_execution());
        let promotable_lane =
            PreviewCertificationLane::from_execution(promotable_execution.as_preview_execution());
        let promotion_parity_lane =
            PreviewCertificationLane::from_execution(promotable_execution.as_preview_execution())
                .with_promotion_parity(&promotion_parity);
        let preview_live_lane =
            PreviewCertificationLane::from_execution(promotable_execution.as_preview_execution())
                .with_preview_live(&preview_live);
        let parity_preview_live_lane = PreviewCertificationLane::from_execution(
            parity_promotable_execution.as_preview_execution(),
        )
        .with_preview_live(&parity_preview_live);
        let preview_live_rebind_lane = PreviewCertificationLane::from_execution(
            preview_live_rebind_preview_execution.as_preview_execution(),
        )
        .with_preview_live_rebind(
            &preview_live_rebind_execution,
            &preview_live_explicit_rebind,
        );

        PreviewCertificationMatrix {
            suite_name: "Preview Session Basis And Promotion Parity Test",
            rows: PREVIEW_CANONICAL_ROW_SPECS
                .iter()
                .map(|spec| {
                    canonical_row(
                        spec,
                        &active_lane,
                        &parity_lane,
                        &promotable_lane,
                        &promotion_parity_lane,
                        &preview_live_lane,
                        &parity_preview_live_lane,
                        &preview_live_rebind_lane,
                    )
                })
                .collect(),
            rejection_rows: PREVIEW_REJECTION_ROW_SPECS
                .iter()
                .map(|spec| {
                    rejection_row(
                        spec,
                        &active_lane,
                        &parity_lane,
                        &unsupported_preview_family,
                        &invalid_basis,
                        &stale_lifecycle,
                        &discarded_lifecycle,
                        &preview_live_drift_denied,
                        &preview_live_broad_fallback_denied,
                        &read_only_writeback_foundation_denied,
                        &promotion_linkage_denied,
                        &replay_linkage_denied,
                        &shape_mismatch_denied,
                        &preview_live_lane,
                        &parity_preview_live_lane,
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
            PREVIEW_CANONICAL_ROW_SPECS, PREVIEW_REJECTION_ROW_SPECS,
            PREVIEW_REQUIRED_CANONICAL_ROW_NAMES, PREVIEW_REQUIRED_REJECTION_ROW_NAMES,
        },
        MilestoneFivePointTwoPreviewCertificationAdapter, PreviewLaneEvaluationClass,
    };
    use crate::harness::certification::{
        milestone_five_point_two_requirements, unmet_required_rows,
    };
    use crate::preview::{
        PreviewBindingCounters, PreviewComparisonCounters, PreviewExecutionCounters,
        PreviewLiveCounters,
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
            spec_missing.is_empty(),
            "preview certification should cover the declared minimum 5.2 spec rows: {spec_missing:?}"
        );
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
                if spec.row_name != "preview-live-drift-explicitness" {
                    assert_ne!(
                        row.control_lane.evaluation_class,
                        row.hostile_lane.evaluation_class
                    );
                }
            } else if !matches!(
                spec.row_name,
                "preview-promotion-comparison-parity"
                    | "preview-comparison-shape-proof-width"
                    | "preview-live-admission-parity"
                    | "preview-workflow-foundation-admission"
                    | "preview-workflow-foundation-no-rescan"
            ) {
                assert_eq!(
                    row.control_lane.evaluation_class,
                    row.hostile_lane.evaluation_class
                );
            }
            match spec.row_name {
                "preview-promotion-comparison-parity"
                | "preview-comparison-shape-proof-width"
                | "preview-live-admission-parity"
                | "preview-live-drift-explicitness"
                | "preview-workflow-foundation-admission"
                | "preview-workflow-foundation-no-rescan"
                | "preview-work-avoided-counter-parity" => {
                    assert_eq!(
                        row.control_lane.evaluation_class,
                        PreviewLaneEvaluationClass::PromotionEligible
                    );
                    assert!(!row.control_lane.workflow_foundation_digest.is_empty());
                }
                _ => {
                    assert_eq!(
                        row.control_lane.evaluation_class,
                        PreviewLaneEvaluationClass::ReadOnly
                    );
                }
            }
            if spec.row_name == "preview-work-avoided-counter-parity" {
                assert_eq!(
                    row.control_lane
                        .execution_counters
                        .preview_work_avoided_by_explicit_basis_count(),
                    1
                );
                assert_eq!(
                    row.hostile_lane
                        .execution_counters
                        .preview_work_avoided_by_explicit_basis_count(),
                    1
                );
                assert_eq!(
                    row.parity_lane
                        .execution_counters
                        .preview_work_avoided_by_explicit_basis_count(),
                    1
                );
            }
            if spec.row_name == "preview-promotion-comparison-parity"
                || spec.row_name == "preview-comparison-shape-proof-width"
            {
                assert!(row.control_lane.promotion_parity_digest.is_some());
                assert!(row.control_lane.comparison_counters.is_some());
            }
            if spec.row_name == "preview-live-admission-parity"
                || spec.row_name == "preview-live-drift-explicitness"
            {
                assert!(row.control_lane.preview_live_digest.is_some());
                assert!(row.control_lane.preview_live_counters.is_some());
            }
            if spec.row_name == "preview-live-drift-explicitness" {
                assert_ne!(
                    row.control_lane.preview_live_digest,
                    row.hostile_lane.preview_live_digest
                );
                assert_eq!(
                    row.hostile_lane
                        .preview_live_counters
                        .as_ref()
                        .expect("rebind lane should retain preview-live counters")
                        .preview_live_rebind_available_count(),
                    1
                );
            }
        }
        for spec in PREVIEW_REJECTION_ROW_SPECS {
            let row = matrix
                .rejection_rows
                .iter()
                .find(|row| row.row_name == spec.row_name)
                .unwrap_or_else(|| panic!("missing preview rejection row {}", spec.row_name));
            assert_eq!(row.perturbation_class, spec.perturbation_class);
            assert_eq!(row.hostile_lane.failure_class, spec.failure_class);
            assert!(
                row.hostile_lane.counters.is_some()
                    || row.hostile_lane.execution_counters.is_some()
                    || row.hostile_lane.comparison_counters.is_some()
                    || row.hostile_lane.preview_live_counters.is_some()
            );
            if spec.row_name == "preview-broad-fallback-forbidden" {
                assert_eq!(
                    row.hostile_lane
                        .counters
                        .as_ref()
                        .expect("broad-fallback denial should retain binding counters")
                        .preview_broad_fallback_denial_count(),
                    1
                );
            }
            if spec.row_name == "read-only-preview-writeback-foundation-forbidden" {
                assert_eq!(
                    row.hostile_lane
                        .execution_counters
                        .as_ref()
                        .expect(
                            "workflow-foundation authority denial should retain execution counters"
                        )
                        .preview_workflow_foundation_denial_count(),
                    1
                );
            }
            if spec.row_name == "preview-live-drift-denied" {
                assert_eq!(
                    row.hostile_lane
                        .preview_live_counters
                        .as_ref()
                        .expect("preview-live drift denial should retain live counters")
                        .preview_live_drift_denial_count(),
                    1
                );
            }
            if spec.row_name == "preview-live-broad-fallback-forbidden" {
                assert_eq!(
                    row.hostile_lane
                        .preview_live_counters
                        .as_ref()
                        .expect("preview-live broad fallback should retain live counters")
                        .preview_live_broad_fallback_denial_count(),
                    1
                );
            }
        }
    }

    #[test]
    fn preview_certification_artifact_reports_offline_ready_completeness() {
        let artifact = MilestoneFivePointTwoPreviewCertificationAdapter::
            preview_session_basis_and_promotion_parity_artifact();
        let mut expected_binding_counters = PreviewBindingCounters::default();
        let mut expected_execution_counters = PreviewExecutionCounters::default();
        let mut expected_comparison_counters = PreviewComparisonCounters::default();
        let mut expected_preview_live_counters = PreviewLiveCounters::default();

        for lane in artifact
            .matrix
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                artifact
                    .matrix
                    .rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
        {
            expected_binding_counters.absorb(&lane.counters);
            expected_execution_counters.absorb(&lane.execution_counters);
            if let Some(comparison_counters) = lane.comparison_counters.as_ref() {
                expected_comparison_counters.absorb(comparison_counters);
            }
            if let Some(preview_live_counters) = lane.preview_live_counters.as_ref() {
                expected_preview_live_counters.absorb(preview_live_counters);
            }
        }

        for rejection in artifact
            .matrix
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.counters.as_ref())
        {
            expected_binding_counters.absorb(rejection);
        }

        for rejection in artifact
            .matrix
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.execution_counters.as_ref())
        {
            expected_execution_counters.absorb(rejection);
        }

        for rejection in artifact
            .matrix
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.comparison_counters.as_ref())
        {
            expected_comparison_counters.absorb(rejection);
        }
        for rejection in artifact
            .matrix
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.preview_live_counters.as_ref())
        {
            expected_preview_live_counters.absorb(rejection);
        }

        assert_eq!(
            artifact.suite_name,
            "Preview Session Basis And Promotion Parity Test"
        );
        assert!(!artifact.certification_bundle_digest.is_empty());
        assert!(!artifact.coverage_matrix_digest.is_empty());
        let requirements = milestone_five_point_two_requirements();
        let missing_spec_rows = unmet_required_rows(
            &artifact.matrix,
            requirements.required_canonical_rows,
            requirements.required_rejection_rows,
        );
        assert_eq!(
            artifact
                .bundle_completeness_report
                .covers_full_milestone_five_point_two_spec_matrix,
            missing_spec_rows.is_empty()
        );
        assert!(
            artifact
                .bundle_completeness_report
                .covers_all_currently_implemented_normative_scenarios
        );
        assert_eq!(
            artifact.bundle_completeness_report.offline_analysis_ready,
            artifact
                .bundle_completeness_report
                .covers_full_milestone_five_point_two_spec_matrix
        );
        assert_eq!(
            artifact
                .bundle_completeness_report
                .zero_rediscovery_lane_count,
            artifact.bundle_completeness_report.supported_lane_count
        );
        assert_eq!(
            artifact
                .bundle_completeness_report
                .preview_live_composition_admitted_by_design,
            true
        );
        assert!(
            expected_preview_live_counters.preview_live_admission_count() > 0,
            "artifact counter snapshot should retain preview-live admissions"
        );
        assert_eq!(
            artifact
                .preview_live_counter_snapshot
                .preview_live_admission_count(),
            expected_preview_live_counters.preview_live_admission_count()
        );
        assert!(
            expected_preview_live_counters.preview_live_execution_count() > 0,
            "artifact counter snapshot should retain preview-live execution counts"
        );
        assert_eq!(
            artifact
                .preview_live_counter_snapshot
                .preview_live_execution_count(),
            expected_preview_live_counters.preview_live_execution_count()
        );
        assert!(
            expected_preview_live_counters.preview_live_drift_denial_count() > 0,
            "artifact counter snapshot should retain preview-live drift denials"
        );
        assert_eq!(
            artifact
                .preview_live_counter_snapshot
                .preview_live_drift_denial_count(),
            expected_preview_live_counters.preview_live_drift_denial_count()
        );
        assert!(
            expected_preview_live_counters.preview_live_rebind_available_count() > 0,
            "artifact counter snapshot should retain preview-live explicit rebinds"
        );
        assert_eq!(
            artifact
                .preview_live_counter_snapshot
                .preview_live_rebind_available_count(),
            expected_preview_live_counters.preview_live_rebind_available_count()
        );
        assert!(
            expected_preview_live_counters.preview_live_broad_fallback_denial_count() > 0,
            "artifact counter snapshot should retain preview-live broad-fallback denials"
        );
        assert_eq!(
            artifact
                .preview_live_counter_snapshot
                .preview_live_broad_fallback_denial_count(),
            expected_preview_live_counters.preview_live_broad_fallback_denial_count()
        );
        assert!(
            expected_binding_counters.preview_invalid_basis_denial_count() > 0,
            "artifact counter snapshot should retain hostile invalid-basis denials"
        );
        assert_eq!(
            artifact
                .binding_counter_snapshot
                .preview_invalid_basis_denial_count(),
            expected_binding_counters.preview_invalid_basis_denial_count()
        );
        assert!(
            expected_binding_counters.preview_broad_fallback_denial_count() > 0,
            "artifact counter snapshot should retain hostile broad-fallback denials"
        );
        assert_eq!(
            artifact
                .binding_counter_snapshot
                .preview_broad_fallback_denial_count(),
            expected_binding_counters.preview_broad_fallback_denial_count()
        );
        assert!(
            expected_binding_counters.preview_invalid_lifecycle_denial_count() > 0,
            "artifact counter snapshot should retain hostile stale-lifecycle denials"
        );
        assert_eq!(
            artifact
                .binding_counter_snapshot
                .preview_invalid_lifecycle_denial_count(),
            expected_binding_counters.preview_invalid_lifecycle_denial_count()
        );
        assert!(
            expected_execution_counters.preview_workflow_foundation_artifact_lookup_count() > 0
        );
        assert_eq!(
            artifact
                .execution_counter_snapshot
                .preview_workflow_foundation_artifact_lookup_count(),
            expected_execution_counters.preview_workflow_foundation_artifact_lookup_count()
        );
        assert_eq!(
            artifact
                .execution_counter_snapshot
                .preview_workflow_foundation_admission_count(),
            expected_execution_counters.preview_workflow_foundation_admission_count()
        );
        assert!(expected_comparison_counters.preview_promotion_comparison_count() > 0);
        assert_eq!(
            artifact
                .comparison_counter_snapshot
                .preview_promotion_comparison_count(),
            expected_comparison_counters.preview_promotion_comparison_count()
        );
        assert!(
            expected_binding_counters.preview_replay_bundle_lookup_count() > 0,
            "artifact counter snapshot should retain replay-linkage lookups"
        );
        assert_eq!(
            artifact
                .binding_counter_snapshot
                .preview_replay_bundle_lookup_count(),
            expected_binding_counters.preview_replay_bundle_lookup_count()
        );
        assert!(
            expected_binding_counters.preview_bridge_promotion_linkage_count() > 0,
            "artifact counter snapshot should retain promotion-linkage lookups"
        );
        assert_eq!(
            artifact
                .binding_counter_snapshot
                .preview_bridge_promotion_linkage_count(),
            expected_binding_counters.preview_bridge_promotion_linkage_count()
        );
        assert_eq!(
            artifact
                .execution_counter_snapshot
                .preview_work_avoided_by_explicit_basis_count(),
            expected_execution_counters.preview_work_avoided_by_explicit_basis_count()
        );
        assert_eq!(
            artifact
                .execution_counter_snapshot
                .preview_workflow_foundation_denial_count(),
            expected_execution_counters.preview_workflow_foundation_denial_count()
        );
    }
}

fn canonical_row(
    spec: &PreviewCanonicalRowSpec,
    active_lane: &PreviewCertificationLane,
    parity_lane: &PreviewCertificationLane,
    promotable_lane: &PreviewCertificationLane,
    promotion_parity_lane: &PreviewCertificationLane,
    preview_live_lane: &PreviewCertificationLane,
    parity_preview_live_lane: &PreviewCertificationLane,
    preview_live_rebind_lane: &PreviewCertificationLane,
) -> CanonicalCertificationRow<PreviewPerturbationClass, PreviewCertificationLane> {
    let control_lane = match spec.row_name {
        "preview-promotion-comparison-parity" | "preview-comparison-shape-proof-width" => {
            promotion_parity_lane.clone()
        }
        "preview-live-admission-parity" | "preview-live-drift-explicitness" => {
            preview_live_lane.clone()
        }
        "preview-workflow-foundation-admission"
        | "preview-workflow-foundation-no-rescan"
        | "preview-work-avoided-counter-parity" => promotable_lane.clone(),
        _ => active_lane.clone(),
    };
    CanonicalCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        hostile_expectation: spec.hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane: control_lane.clone(),
        hostile_lane: match spec.hostile_lane_selector {
            row_catalog::PreviewLaneSelector::ParityExecution => parity_lane.clone(),
            row_catalog::PreviewLaneSelector::PromotionEligibleExecution => promotable_lane.clone(),
            row_catalog::PreviewLaneSelector::PromotionParity => promotion_parity_lane.clone(),
            row_catalog::PreviewLaneSelector::PreviewLiveAdmission => {
                parity_preview_live_lane.clone()
            }
            row_catalog::PreviewLaneSelector::PreviewLiveRebind => preview_live_rebind_lane.clone(),
        },
        parity_lane: control_lane,
    }
}

fn rejection_row(
    spec: &PreviewRejectionRowSpec,
    active_lane: &PreviewCertificationLane,
    parity_lane: &PreviewCertificationLane,
    unsupported_preview_family: &PreviewBindingError,
    invalid_basis: &PreviewBindingError,
    stale_lifecycle: &PreviewBindingError,
    discarded_lifecycle: &PreviewBindingError,
    preview_live_drift_denied: &crate::preview::PreviewLiveDriftDenied,
    preview_live_broad_fallback_denied: &crate::preview::PreviewLiveDriftDenied,
    read_only_writeback_foundation_denied: &crate::preview::PreviewWorkflowFoundationError,
    promotion_linkage_denied: &PreviewBindingError,
    replay_linkage_denied: &PreviewBindingError,
    shape_mismatch_denied: &PreviewComparisonError,
    preview_live_lane: &PreviewCertificationLane,
    parity_preview_live_lane: &PreviewCertificationLane,
) -> RejectionCertificationRow<
    PreviewPerturbationClass,
    PreviewCertificationLane,
    PreviewCertificationRejection,
> {
    let hostile_lane = match spec.runtime_failure_selector {
        Some(row_catalog::PreviewRuntimeFailureSelector::UnsupportedPreviewFamily) => {
            PreviewCertificationRejection::from_runtime_failure(
                unsupported_preview_family.failure_class(),
                unsupported_preview_family.counters(),
            )
        }
        Some(row_catalog::PreviewRuntimeFailureSelector::InvalidBasis) => {
            PreviewCertificationRejection::from_runtime_failure(
                invalid_basis.failure_class(),
                invalid_basis.counters(),
            )
        }
        Some(row_catalog::PreviewRuntimeFailureSelector::BroadFallbackDenied) => {
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
        Some(row_catalog::PreviewRuntimeFailureSelector::DiscardedLifecycle) => {
            PreviewCertificationRejection::from_runtime_failure(
                discarded_lifecycle.failure_class(),
                discarded_lifecycle.counters(),
            )
        }
        Some(row_catalog::PreviewRuntimeFailureSelector::PreviewLiveDriftDenied) => {
            PreviewCertificationRejection::from_preview_live_failure(
                preview_live_drift_denied.error(),
            )
        }
        Some(row_catalog::PreviewRuntimeFailureSelector::PreviewLiveBroadFallbackDenied) => {
            PreviewCertificationRejection::from_preview_live_failure(
                preview_live_broad_fallback_denied.error(),
            )
        }
        Some(row_catalog::PreviewRuntimeFailureSelector::WorkflowFoundationAuthorityDenied) => {
            PreviewCertificationRejection::from_workflow_failure(
                read_only_writeback_foundation_denied,
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
        Some(row_catalog::PreviewRuntimeFailureSelector::ShapeMismatchDenied) => {
            PreviewCertificationRejection::from_comparison_failure(shape_mismatch_denied)
        }
        None => panic!(
            "preview rejection row {} has no runtime denial",
            spec.row_name
        ),
    };

    RejectionCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        control_lane: match spec.runtime_failure_selector {
            Some(row_catalog::PreviewRuntimeFailureSelector::PreviewLiveDriftDenied)
            | Some(row_catalog::PreviewRuntimeFailureSelector::PreviewLiveBroadFallbackDenied) => {
                preview_live_lane.clone()
            }
            _ => active_lane.clone(),
        },
        hostile_lane,
        parity_lane: match spec.runtime_failure_selector {
            Some(row_catalog::PreviewRuntimeFailureSelector::PreviewLiveDriftDenied)
            | Some(row_catalog::PreviewRuntimeFailureSelector::PreviewLiveBroadFallbackDenied) => {
                parity_preview_live_lane.clone()
            }
            _ => parity_lane.clone(),
        },
    }
}
