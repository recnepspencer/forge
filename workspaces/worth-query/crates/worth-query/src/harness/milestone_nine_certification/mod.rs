#[cfg(test)]
mod tests;

use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RawAuthoredQuery,
    RawAuthoredResultShape, RootEntityKey, TraversalSelector,
};
use crate::harness::certification::{
    digest_parts, CanonicalCertificationRow, CertificationMatrix, HostileExpectation, ParityAnchor,
    RejectionCertificationRow,
};
use crate::policy_basis::{
    admit_policy_tenant_context, classify_saved_query_policy_tenant_reuse,
    runtime_backed_policy_tenant_admission_support_profile, BranchAccessGrant,
    PolicyAdmissionDisposition, PolicyCostPosture, PolicyEpoch, PolicyExecutionModeRequest,
    PolicyRuleSnapshot, PolicyTenantAdmissionFailureClass, PolicyWorkBudget,
    SavedQueryPolicyReuseDescriptor, SavedQueryPolicyReuseDisposition,
};
use crate::policy_narrowing::narrow_policy_query;
use crate::policy_narrowing::{
    classify_saved_policy_narrowing_reuse, SavedPolicyNarrowingReuseDescriptor,
};
use crate::relationship_proof::{
    RelationshipProofBudget, RelationshipProofDescriptor, RelationshipProofDescriptorSet,
};
use crate::tenant_basis::{SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot};

pub const MILESTONE_NINE_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "current-read-policy-tenant-admission",
    "branch-read-policy-tenant-admission",
    "historical-read-policy-tenant-admission",
    "policy-narrowing-disposition",
    "policy-work-budget-explicitness",
    "saved-query-exact-policy-tenant-reuse",
    "support-profile-honesty",
    "authorized-projection-removes-masked-aspect",
    "non-disclosing-use-is-not-delivered",
    "relationship-proof-direct-edge-admission",
    "relationship-proof-tenant-membership-admission",
    "narrowed-artifact-binds-policy-tenant-schema",
    "optimizer-input-excludes-masked-fields",
    "saved-query-exact-reuse-narrows-identically",
    "phase-two-support-profile-honesty",
    "policy-aware-current-plan-lowering",
    "policy-aware-branch-plan-lowering",
    "policy-aware-historical-plan-runtime-backed-lowering",
    "policy-aware-diff-plan-runtime-backed-lowering",
    "policy-aware-live-admission",
    "policy-aware-delivery-shape-derived-after-mask",
    "policy-aware-optimizer-input-only",
    "policy-execution-seam-parity",
    "policy-execution-handoff-honesty",
    "employee-record-fixture-policy-basis",
    "tenant-alpha-versus-tenant-beta-schema",
    "masked-versus-unmasked-policy-parity",
    "delivery-width-class-honesty",
    "live-policy-epoch-drift-readmission",
    "live-policy-density-posture-honesty",
    "policy-scale-slope-honesty",
    "policy-direct-scope-template-saved-parity",
    "policy-view-shape-delivery-parity",
    "policy-identity-aware-inspector-parity",
];

pub const MILESTONE_NINE_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "live-subscription-deferred-before-truth",
    "historical-diff-deferred-before-truth",
    "unknown-policy-cost-denied-before-truth",
    "branch-denial-before-tenant-truth",
    "hidden-tenant-filter-denied",
    "global-schema-fallback-denied",
    "saved-query-policy-tenant-drift",
    "masked-predicate-denies-before-narrowing",
    "masked-ordering-denies-before-narrowing",
    "masked-grouping-denies-before-narrowing",
    "relationship-proof-host-callback-forbidden",
    "relationship-proof-unbounded-recursion-denied",
    "relationship-proof-query-conflict-denied",
    "template-hidden-influence-denied",
    "saved-query-policy-drift-renarrowing-required",
    "unknown-narrowing-cost-denied-before-truth",
    "phase-two-no-truth-touch",
    "raw-current-plan-bypass-forbidden",
    "raw-branch-plan-bypass-forbidden",
    "raw-historical-plan-bypass-forbidden",
    "raw-diff-scrub-forbidden",
    "masked-live-relevance-forbidden",
    "delivery-shape-overexposure-forbidden",
    "store-backed-policy-execution-deferred",
    "durable-policy-cursor-deferred",
    "durable-policy-artifact-reload-deferred",
    "durable-policy-delivery-metadata-deferred",
    "phase-three-no-truth-touch-before-plan-admission",
    "masked-placeholder-shape-forbidden",
    "masked-aggregation-without-witness-forbidden",
    "masked-cursor-without-witness-forbidden",
    "masked-view-membership-without-witness-forbidden",
    "policy-per-row-allocation-forbidden",
    "policy-cross-tenant-fanout-forbidden",
    "saved-query-policy-bypass-forbidden",
    "unsupported-policy-workflow-composition-forbidden",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MilestoneNinePerturbationClass {
    CurrentReadAdmission,
    BranchReadAdmission,
    HistoricalReadAdmission,
    PolicyNarrowingDisposition,
    PolicyWorkBudgetExplicitness,
    SavedQueryExactReuse,
    SupportProfileHonesty,
    LiveSubscriptionDeferred,
    HistoricalDiffDeferred,
    UnknownPolicyCostDenied,
    BranchDeniedBeforeTruth,
    HiddenTenantFilterDenied,
    GlobalSchemaFallbackDenied,
    SavedQueryPolicyTenantDrift,
    AuthorizedProjectionRemovesMaskedAspect,
    RelationshipProofDirectEdgeAdmission,
    RelationshipProofTenantMembershipAdmission,
    NarrowedArtifactBindsPolicyTenantSchema,
    OptimizerInputExcludesMaskedFields,
    SavedQueryExactReuseNarrowsIdentically,
    PhaseTwoSupportProfileHonesty,
    NonDisclosingUseIsNotDelivered,
    MaskedPredicateDeniedBeforeNarrowing,
    MaskedOrderingDeniedBeforeNarrowing,
    MaskedGroupingDeniedBeforeNarrowing,
    RelationshipProofHostCallbackForbidden,
    RelationshipProofUnboundedRecursionDenied,
    RelationshipProofQueryConflictDenied,
    TemplateHiddenInfluenceDenied,
    SavedQueryPolicyDriftRenarrowingRequired,
    UnknownNarrowingCostDenied,
    PhaseTwoNoTruthTouch,
    PolicyAwareCurrentPlanLowering,
    PolicyAwareBranchPlanLowering,
    PolicyAwareHistoricalPlanLowering,
    PolicyAwareDiffPlanLowering,
    PolicyAwareLiveAdmission,
    PolicyAwareDeliveryShapeDerivedAfterMask,
    PolicyAwareOptimizerInputOnly,
    PolicyExecutionSeamParity,
    RawCurrentPlanBypassForbidden,
    RawBranchPlanBypassForbidden,
    RawHistoricalPlanBypassForbidden,
    RawDiffScrubForbidden,
    MaskedLiveRelevanceForbidden,
    DeliveryShapeOverexposureForbidden,
    StoreBackedPolicyExecutionDeferred,
    DurablePolicyCursorDeferred,
    DurablePolicyArtifactReloadDeferred,
    DurablePolicyDeliveryMetadataDeferred,
    PhaseThreeNoTruthTouch,
    PolicyExecutionHandoffHonesty,
    EmployeeRecordFixturePolicyBasis,
    TenantAlphaVersusTenantBetaSchema,
    MaskedVersusUnmaskedPolicyParity,
    DeliveryWidthClassHonesty,
    LivePolicyEpochDriftReadmission,
    LivePolicyDensityPostureHonesty,
    PolicyScaleSlopeHonesty,
    PolicyDirectScopeTemplateSavedParity,
    PolicyViewShapeDeliveryParity,
    PolicyIdentityAwareInspectorParity,
    MaskedPlaceholderShapeForbidden,
    MaskedAggregationWithoutWitnessForbidden,
    MaskedCursorWithoutWitnessForbidden,
    MaskedViewMembershipWithoutWitnessForbidden,
    PolicyPerRowAllocationForbidden,
    PolicyCrossTenantFanoutForbidden,
    SavedQueryPolicyBypassForbidden,
    UnsupportedPolicyWorkflowCompositionForbidden,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneNineFailureClass {
    UnsupportedExecutionMode,
    BranchAccessDenied,
    TenantAdmissionDenied,
    SavedQueryPolicyTenantDrift,
    PolicyNarrowingDenied,
    RelationshipProofDenied,
    PolicyExecutionSeamDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNineCertificationBundle {
    pub canonical_query_digest: String,
    pub policy_digest: String,
    pub result_digest: String,
    pub tenant_truth_basis_digest: String,
    pub tenant_schema_basis_digest: String,
    pub branch_access_digest: String,
    pub schema_variant_digest: String,
    pub execution_mode: String,
    pub admission_disposition: String,
    pub policy_cost_posture: String,
    pub policy_work_budget_digest: String,
    pub authorized_projection_digest: String,
    pub narrowed_result_shape_digest: String,
    pub relationship_proof_digest: String,
    pub validation_report_digest: String,
    pub policy_plan_digest: String,
    pub policy_execution_seam_digest: String,
    pub delivery_digest: String,
    pub employee_fixture_digest: String,
    pub policy_scale_counter_slope_digest: String,
    pub live_drift_evidence_digest: String,
    pub delivery_width_class_digest: String,
    pub composition_policy_parity_digest: String,
    pub view_shape_policy_parity_digest: String,
    pub placeholder_denial_digest: String,
    pub counter_snapshot_digest: String,
    pub support_profile_digest: String,
}

impl MilestoneNineCertificationBundle {
    fn has_required_outputs(&self) -> bool {
        !self.canonical_query_digest.is_empty()
            && !self.policy_digest.is_empty()
            && !self.result_digest.is_empty()
            && !self.tenant_truth_basis_digest.is_empty()
            && !self.tenant_schema_basis_digest.is_empty()
            && !self.branch_access_digest.is_empty()
            && !self.schema_variant_digest.is_empty()
            && !self.policy_cost_posture.is_empty()
            && !self.policy_work_budget_digest.is_empty()
            && !self.authorized_projection_digest.is_empty()
            && !self.narrowed_result_shape_digest.is_empty()
            && !self.relationship_proof_digest.is_empty()
            && !self.validation_report_digest.is_empty()
            && !self.policy_plan_digest.is_empty()
            && !self.policy_execution_seam_digest.is_empty()
            && !self.delivery_digest.is_empty()
            && !self.employee_fixture_digest.is_empty()
            && !self.policy_scale_counter_slope_digest.is_empty()
            && !self.live_drift_evidence_digest.is_empty()
            && !self.delivery_width_class_digest.is_empty()
            && !self.composition_policy_parity_digest.is_empty()
            && !self.view_shape_policy_parity_digest.is_empty()
            && !self.placeholder_denial_digest.is_empty()
            && !self.counter_snapshot_digest.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNineRejectionBundle {
    pub failure_class: MilestoneNineFailureClass,
    pub failure_digest: String,
    pub counter_snapshot_digest: String,
}

pub type MilestoneNineCertificationRow =
    CanonicalCertificationRow<MilestoneNinePerturbationClass, MilestoneNineCertificationBundle>;
pub type MilestoneNineRejectionRow = RejectionCertificationRow<
    MilestoneNinePerturbationClass,
    MilestoneNineCertificationBundle,
    MilestoneNineRejectionBundle,
>;
pub type MilestoneNineCertificationMatrix = CertificationMatrix<
    MilestoneNinePerturbationClass,
    MilestoneNineCertificationBundle,
    MilestoneNineRejectionBundle,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNineCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub matrix: MilestoneNineCertificationMatrix,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MilestoneNinePhaseFourSupportSurface {
    EmployeeRecordFixture,
    HiddenInfluenceExhaustiveness,
    PlaceholderMaskingDenial,
    LiveDriftReadmission,
    DeliveryWidthClass,
    PolicyScaleSlope,
    PolicyCompositionParity,
    StoreBackedPolicyExecution,
    DurablePolicyArtifacts,
}

impl MilestoneNinePhaseFourSupportSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmployeeRecordFixture => "employee_record_fixture",
            Self::HiddenInfluenceExhaustiveness => "hidden_influence_exhaustiveness",
            Self::PlaceholderMaskingDenial => "placeholder_masking_denial",
            Self::LiveDriftReadmission => "live_drift_readmission",
            Self::DeliveryWidthClass => "delivery_width_class",
            Self::PolicyScaleSlope => "policy_scale_slope",
            Self::PolicyCompositionParity => "policy_composition_parity",
            Self::StoreBackedPolicyExecution => "store_backed_policy_execution",
            Self::DurablePolicyArtifacts => "durable_policy_artifacts",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MilestoneNinePhaseFourSupportStatus {
    Verified,
    Deferred,
}

impl MilestoneNinePhaseFourSupportStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNinePhaseFourDiagnostic {
    surface: MilestoneNinePhaseFourSupportSurface,
    status: MilestoneNinePhaseFourSupportStatus,
    row_name: &'static str,
}

impl MilestoneNinePhaseFourDiagnostic {
    fn new(
        surface: MilestoneNinePhaseFourSupportSurface,
        status: MilestoneNinePhaseFourSupportStatus,
        row_name: &'static str,
    ) -> Self {
        Self {
            surface,
            status,
            row_name,
        }
    }

    pub fn surface(&self) -> MilestoneNinePhaseFourSupportSurface {
        self.surface
    }

    pub fn status(&self) -> MilestoneNinePhaseFourSupportStatus {
        self.status
    }

    pub fn row_name(&self) -> &'static str {
        self.row_name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNinePhaseFourSupportReport {
    diagnostics: Vec<MilestoneNinePhaseFourDiagnostic>,
    report_digest: String,
}

impl MilestoneNinePhaseFourSupportReport {
    fn new(matrix: &MilestoneNineCertificationMatrix) -> Self {
        fn phase_four_value_verified(value: &str) -> bool {
            !value.is_empty() && !value.contains("deferred")
        }

        fn canonical_verified(
            matrix: &MilestoneNineCertificationMatrix,
            row_name: &str,
            evidence: impl Fn(&MilestoneNineCertificationBundle) -> bool,
        ) -> bool {
            matrix
                .rows
                .iter()
                .find(|row| row.row_name == row_name)
                .is_some_and(|row| {
                    row.control_lane.has_required_outputs()
                        && row.hostile_lane.has_required_outputs()
                        && row.parity_lane.has_required_outputs()
                        && evidence(&row.control_lane)
                        && evidence(&row.hostile_lane)
                        && evidence(&row.parity_lane)
                })
        }

        fn rejection_verified(
            matrix: &MilestoneNineCertificationMatrix,
            row_name: &str,
            expected_failure: MilestoneNineFailureClass,
        ) -> bool {
            matrix
                .rejection_rows
                .iter()
                .find(|row| row.row_name == row_name)
                .is_some_and(|row| {
                    row.control_lane.has_required_outputs()
                        && row.parity_lane.has_required_outputs()
                        && row.hostile_lane.failure_class == expected_failure
                        && !row.hostile_lane.failure_digest.is_empty()
                        && !row.hostile_lane.counter_snapshot_digest.is_empty()
                })
        }

        let candidates = [
            (
                MilestoneNinePhaseFourSupportSurface::EmployeeRecordFixture,
                MilestoneNinePhaseFourSupportStatus::Verified,
                "employee-record-fixture-policy-basis",
                canonical_verified(matrix, "employee-record-fixture-policy-basis", |bundle| {
                    phase_four_value_verified(&bundle.employee_fixture_digest)
                }),
            ),
            (
                MilestoneNinePhaseFourSupportSurface::HiddenInfluenceExhaustiveness,
                MilestoneNinePhaseFourSupportStatus::Verified,
                "masked-view-membership-without-witness-forbidden",
                rejection_verified(
                    matrix,
                    "masked-aggregation-without-witness-forbidden",
                    MilestoneNineFailureClass::PolicyNarrowingDenied,
                ) && rejection_verified(
                    matrix,
                    "masked-cursor-without-witness-forbidden",
                    MilestoneNineFailureClass::PolicyNarrowingDenied,
                ) && rejection_verified(
                    matrix,
                    "masked-view-membership-without-witness-forbidden",
                    MilestoneNineFailureClass::PolicyNarrowingDenied,
                ),
            ),
            (
                MilestoneNinePhaseFourSupportSurface::PlaceholderMaskingDenial,
                MilestoneNinePhaseFourSupportStatus::Verified,
                "masked-placeholder-shape-forbidden",
                rejection_verified(
                    matrix,
                    "masked-placeholder-shape-forbidden",
                    MilestoneNineFailureClass::PolicyExecutionSeamDenied,
                ),
            ),
            (
                MilestoneNinePhaseFourSupportSurface::LiveDriftReadmission,
                MilestoneNinePhaseFourSupportStatus::Verified,
                "live-policy-epoch-drift-readmission",
                canonical_verified(matrix, "live-policy-epoch-drift-readmission", |bundle| {
                    phase_four_value_verified(&bundle.live_drift_evidence_digest)
                }) && canonical_verified(matrix, "live-policy-density-posture-honesty", |bundle| {
                    phase_four_value_verified(&bundle.live_drift_evidence_digest)
                }),
            ),
            (
                MilestoneNinePhaseFourSupportSurface::DeliveryWidthClass,
                MilestoneNinePhaseFourSupportStatus::Verified,
                "delivery-width-class-honesty",
                canonical_verified(matrix, "delivery-width-class-honesty", |bundle| {
                    phase_four_value_verified(&bundle.delivery_width_class_digest)
                }),
            ),
            (
                MilestoneNinePhaseFourSupportSurface::PolicyScaleSlope,
                MilestoneNinePhaseFourSupportStatus::Verified,
                "policy-scale-slope-honesty",
                canonical_verified(matrix, "policy-scale-slope-honesty", |bundle| {
                    phase_four_value_verified(&bundle.policy_scale_counter_slope_digest)
                }),
            ),
            (
                MilestoneNinePhaseFourSupportSurface::PolicyCompositionParity,
                MilestoneNinePhaseFourSupportStatus::Verified,
                "policy-direct-scope-template-saved-parity",
                canonical_verified(
                    matrix,
                    "policy-direct-scope-template-saved-parity",
                    |bundle| phase_four_value_verified(&bundle.composition_policy_parity_digest),
                ) && canonical_verified(matrix, "policy-view-shape-delivery-parity", |bundle| {
                    phase_four_value_verified(&bundle.view_shape_policy_parity_digest)
                }) && canonical_verified(
                    matrix,
                    "policy-identity-aware-inspector-parity",
                    |bundle| phase_four_value_verified(&bundle.view_shape_policy_parity_digest),
                ),
            ),
            (
                MilestoneNinePhaseFourSupportSurface::StoreBackedPolicyExecution,
                MilestoneNinePhaseFourSupportStatus::Deferred,
                "store-backed-policy-execution-deferred",
                rejection_verified(
                    matrix,
                    "store-backed-policy-execution-deferred",
                    MilestoneNineFailureClass::PolicyExecutionSeamDenied,
                ),
            ),
            (
                MilestoneNinePhaseFourSupportSurface::DurablePolicyArtifacts,
                MilestoneNinePhaseFourSupportStatus::Deferred,
                "durable-policy-artifact-reload-deferred",
                rejection_verified(
                    matrix,
                    "durable-policy-cursor-deferred",
                    MilestoneNineFailureClass::PolicyExecutionSeamDenied,
                ) && rejection_verified(
                    matrix,
                    "durable-policy-artifact-reload-deferred",
                    MilestoneNineFailureClass::PolicyExecutionSeamDenied,
                ) && rejection_verified(
                    matrix,
                    "durable-policy-delivery-metadata-deferred",
                    MilestoneNineFailureClass::PolicyExecutionSeamDenied,
                ),
            ),
        ];
        let diagnostics = candidates
            .into_iter()
            .filter(|(_, _, _, present)| *present)
            .map(|(surface, status, row_name, _)| {
                MilestoneNinePhaseFourDiagnostic::new(surface, status, row_name)
            })
            .collect::<Vec<_>>();
        let report_digest = digest_parts(
            &diagnostics
                .iter()
                .map(|diagnostic| {
                    format!(
                        "{}:{}:{}",
                        diagnostic.surface().as_str(),
                        diagnostic.status().as_str(),
                        diagnostic.row_name()
                    )
                })
                .collect::<Vec<_>>(),
        );
        Self {
            diagnostics,
            report_digest,
        }
    }

    pub fn diagnostics(&self) -> &[MilestoneNinePhaseFourDiagnostic] {
        &self.diagnostics
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn verified_surface_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.status() == MilestoneNinePhaseFourSupportStatus::Verified
            })
            .count()
    }

    pub fn deferred_surface_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.status() == MilestoneNinePhaseFourSupportStatus::Deferred
            })
            .count()
    }
}

impl MilestoneNineCertificationMatrix {
    pub fn into_milestone_nine_artifact(self) -> MilestoneNineCertificationArtifact {
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));
        MilestoneNineCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            matrix: self,
        }
    }

    pub fn phase_four_support_report(&self) -> MilestoneNinePhaseFourSupportReport {
        MilestoneNinePhaseFourSupportReport::new(self)
    }
}

pub struct MilestoneNineCertificationAdapter;

impl MilestoneNineCertificationAdapter {
    pub fn policy_tenant_context_admission_certification_artifact(
    ) -> MilestoneNineCertificationArtifact {
        Self::policy_tenant_context_admission_test().into_milestone_nine_artifact()
    }

    pub fn policy_tenant_context_admission_test() -> MilestoneNineCertificationMatrix {
        MilestoneNineCertificationMatrix {
            suite_name: "Policy And Tenant Context Admission Test",
            rows: canonical_rows(),
            rejection_rows: rejection_rows(),
        }
    }
}

fn canonical_query() -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

fn canonical_query_with_secret_projection() -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .project(AspectFieldSelector::new("secret", "salary").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

fn canonical_query_with_manager_traversal() -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .project(AspectFieldSelector::new("secret", "salary").unwrap())
        .traverse(TraversalSelector::bounded("manager", 1).unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

fn canonical_query_with_secret_predicate() -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .where_equal(
            crate::authoring::EqualityPredicate::new(
                "secret",
                "salary",
                crate::authoring::WorthQueryPredicateOperand::int64(7),
            )
            .unwrap(),
        )
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

fn canonical_query_with_secret_ordering() -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .order_by(crate::authoring::OrderingSelector::ascending("secret", "salary").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

fn base_policy(narrowed: bool) -> PolicyRuleSnapshot {
    if narrowed {
        PolicyRuleSnapshot::synthetic_authority_with_projection(
            "runtime-policy",
            "rules-v1",
            PolicyEpoch::Synthetic(7),
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        )
    } else {
        PolicyRuleSnapshot::synthetic_authority(
            "runtime-policy",
            "rules-v1",
            PolicyEpoch::Synthetic(7),
        )
    }
}

fn tenant() -> TenantBindingSnapshot {
    TenantBindingSnapshot::synthetic_direct(
        "tenant-a",
        "branch-a",
        "schema-a",
        TenantBasisEpoch::Synthetic(3),
    )
}

fn schema() -> SchemaVariantSnapshot {
    SchemaVariantSnapshot::synthetic_authority("tenant-a", "schema-a", "compatible")
}

fn admitted_bundle(
    mode: PolicyExecutionModeRequest,
    narrowed: bool,
) -> MilestoneNineCertificationBundle {
    let canonical = canonical_query();
    let policy = base_policy(narrowed);
    let branch = BranchAccessGrant::synthetic_granted("branch-a", &policy);
    let schema = schema();
    let admitted =
        admit_policy_tenant_context(canonical.query(), policy, tenant(), branch, schema, mode)
            .unwrap();
    let support_profile = runtime_backed_policy_tenant_admission_support_profile();

    MilestoneNineCertificationBundle {
        canonical_query_digest: admitted.bundle().canonical_query_digest().to_string(),
        policy_digest: admitted.bundle().policy_digest().to_string(),
        result_digest: digest_parts(&[
            format!("query:{}", admitted.bundle().canonical_query_digest()),
            format!("policy:{}", admitted.bundle().policy_digest()),
            format!("mode:{}", admitted.bundle().execution_mode().as_str()),
        ]),
        tenant_truth_basis_digest: admitted.bundle().tenant_truth_basis_digest().to_string(),
        tenant_schema_basis_digest: admitted.bundle().tenant_schema_basis_digest().to_string(),
        branch_access_digest: admitted.bundle().branch_access_digest().to_string(),
        schema_variant_digest: admitted.bundle().schema_variant_digest().to_string(),
        execution_mode: admitted.bundle().execution_mode().as_str().to_string(),
        admission_disposition: admitted
            .bundle()
            .admission_disposition()
            .as_str()
            .to_string(),
        policy_cost_posture: admitted.bundle().policy_cost_posture().as_str().to_string(),
        policy_work_budget_digest: admitted.bundle().policy_work_budget().digest_part(),
        authorized_projection_digest: "phase1-authorized-projection-deferred".to_string(),
        narrowed_result_shape_digest: "phase1-narrowed-result-shape-deferred".to_string(),
        relationship_proof_digest: "phase1-relationship-proof-deferred".to_string(),
        validation_report_digest: "phase1-validation-report-deferred".to_string(),
        policy_plan_digest: "phase1-policy-plan-deferred".to_string(),
        policy_execution_seam_digest: "phase1-policy-seam-deferred".to_string(),
        delivery_digest: "phase1-delivery-deferred".to_string(),
        employee_fixture_digest: "phase1-employee-fixture-deferred".to_string(),
        policy_scale_counter_slope_digest: "phase1-policy-scale-deferred".to_string(),
        live_drift_evidence_digest: "phase1-live-drift-deferred".to_string(),
        delivery_width_class_digest: "phase1-delivery-width-deferred".to_string(),
        composition_policy_parity_digest: "phase1-composition-parity-deferred".to_string(),
        view_shape_policy_parity_digest: "phase1-view-shape-parity-deferred".to_string(),
        placeholder_denial_digest: "phase1-placeholder-denial-deferred".to_string(),
        counter_snapshot_digest: digest_parts(&[
            format!(
                "policy:{}",
                admitted
                    .bundle()
                    .counters()
                    .policy()
                    .policy_basis_admitted_count()
            ),
            format!(
                "tenant:{}",
                admitted
                    .bundle()
                    .counters()
                    .tenant()
                    .direct_tenant_binding_admitted_count()
            ),
            format!(
                "bundle:{}",
                admitted.bundle().counters().admission_bundle_count()
            ),
        ]),
        support_profile_digest: support_profile.profile_digest().to_string(),
    }
}

fn phase_two_bundle(
    canonical: crate::canonicalization::CanonicalQueryBundle,
    mask: crate::authorized_projection::PolicyAspectMask,
    descriptors: RelationshipProofDescriptorSet,
) -> MilestoneNineCertificationBundle {
    let policy = base_policy(true);
    let branch = BranchAccessGrant::synthetic_granted("branch-a", &policy);
    let admitted = admit_policy_tenant_context(
        canonical.query(),
        policy,
        tenant(),
        branch,
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    let mask = crate::authorized_projection::PolicyMaskSnapshot::synthetic_authority(
        admitted.bundle().policy_digest(),
        mask,
    );
    let narrowed = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask,
        crate::authorized_projection::PolicyInfluenceSet::none(),
        descriptors,
    )
    .unwrap();
    let support_profile =
        crate::policy_narrowing::runtime_backed_policy_narrowing_support_profile();

    MilestoneNineCertificationBundle {
        canonical_query_digest: narrowed.canonical_query_digest().to_string(),
        policy_digest: admitted.bundle().policy_digest().to_string(),
        result_digest: digest_parts(&[
            format!("narrowed:{}", narrowed.digest()),
            format!("shape:{}", narrowed.narrowed_result_shape_digest()),
            format!(
                "authorized_projection:{}",
                narrowed.authorized_projection().identity().as_str()
            ),
        ]),
        tenant_truth_basis_digest: admitted.bundle().tenant_truth_basis_digest().to_string(),
        tenant_schema_basis_digest: admitted.bundle().tenant_schema_basis_digest().to_string(),
        branch_access_digest: admitted.bundle().branch_access_digest().to_string(),
        schema_variant_digest: admitted.bundle().schema_variant_digest().to_string(),
        execution_mode: admitted.bundle().execution_mode().as_str().to_string(),
        admission_disposition: admitted
            .bundle()
            .admission_disposition()
            .as_str()
            .to_string(),
        policy_cost_posture: narrowed.cost_posture().as_str().to_string(),
        policy_work_budget_digest: narrowed.work_budget().digest_part(),
        authorized_projection_digest: narrowed
            .authorized_projection()
            .identity()
            .as_str()
            .to_string(),
        narrowed_result_shape_digest: narrowed.narrowed_result_shape_digest().to_string(),
        relationship_proof_digest: narrowed
            .relationship_proof()
            .identity()
            .as_str()
            .to_string(),
        validation_report_digest: narrowed.validation_report().digest().to_string(),
        policy_plan_digest: "phase2-policy-plan-deferred".to_string(),
        policy_execution_seam_digest: "phase2-policy-seam-deferred".to_string(),
        delivery_digest: "phase2-delivery-deferred".to_string(),
        employee_fixture_digest: "phase2-employee-fixture-deferred".to_string(),
        policy_scale_counter_slope_digest: "phase2-policy-scale-deferred".to_string(),
        live_drift_evidence_digest: "phase2-live-drift-deferred".to_string(),
        delivery_width_class_digest: "phase2-delivery-width-deferred".to_string(),
        composition_policy_parity_digest: "phase2-composition-parity-deferred".to_string(),
        view_shape_policy_parity_digest: "phase2-view-shape-parity-deferred".to_string(),
        placeholder_denial_digest: "phase2-placeholder-denial-deferred".to_string(),
        counter_snapshot_digest: narrowed
            .validation_report()
            .counter_snapshot_digest()
            .to_string(),
        support_profile_digest: support_profile.profile_digest().to_string(),
    }
}

fn phase_two_mask_snapshot(
    admitted: &crate::policy_basis::AdmittedPolicyTenantContext,
    mask: crate::authorized_projection::PolicyAspectMask,
) -> crate::authorized_projection::PolicyMaskSnapshot {
    crate::authorized_projection::PolicyMaskSnapshot::synthetic_authority(
        admitted.bundle().policy_digest(),
        mask,
    )
}

fn secret_salary_key() -> crate::authoring::AspectFieldKey {
    crate::authoring::AspectFieldKey::from_authoring_parts("secret", "salary").unwrap()
}

pub(crate) fn phase_three_test_narrowed_artifact(
) -> crate::policy_narrowing::NarrowedPolicyQueryArtifact {
    let canonical = canonical_query_with_secret_projection();
    let policy = base_policy(true);
    let branch = BranchAccessGrant::synthetic_granted("branch-a", &policy);
    let admitted = admit_policy_tenant_context(
        canonical.query(),
        policy,
        tenant(),
        branch,
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    narrow_policy_query(
        &canonical,
        admitted.clone(),
        phase_two_mask_snapshot(
            &admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap()
}

fn phase_three_test_unmasked_artifact() -> crate::policy_narrowing::NarrowedPolicyQueryArtifact {
    let canonical = canonical_query_with_secret_projection();
    let policy = base_policy(false);
    let branch = BranchAccessGrant::synthetic_granted("branch-a", &policy);
    let admitted = admit_policy_tenant_context(
        canonical.query(),
        policy,
        tenant(),
        branch,
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    narrow_policy_query(
        &canonical,
        admitted.clone(),
        phase_two_mask_snapshot(
            &admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all(),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap()
}

fn native_authorized_projection_fields(
    narrowed: &crate::policy_narrowing::NarrowedPolicyQueryArtifact,
) -> Vec<crate::authorized_projection::AuthorizedProjectionFieldPath> {
    narrowed
        .authorized_projection()
        .visible_field_paths()
        .to_vec()
}

fn authorized_projection_field(
    aspect: &str,
    field: &str,
) -> crate::authorized_projection::AuthorizedProjectionFieldPath {
    crate::authorized_projection::AuthorizedProjectionFieldPath::from_native_keys(
        worth_foundational::facade::AspectKey::new(aspect.to_string())
            .expect("certification aspect key"),
        worth_foundational::facade::FieldKey::new(field.to_string())
            .expect("certification field key"),
    )
}

fn policy_placeholder_request(
    fields: impl IntoIterator<Item = (&'static str, &'static str)>,
) -> crate::policy_delivery::PolicyPlaceholderMaskingRequest {
    let fields = fields
        .into_iter()
        .map(|(aspect, field)| {
            crate::authorized_projection::AuthorizedProjectionFieldPath::from_native_keys(
                worth_foundational::facade::AspectKey::new(aspect)
                    .expect("placeholder aspect key should admit"),
                worth_foundational::facade::FieldKey::new(field)
                    .expect("placeholder field key should admit"),
            )
        })
        .collect();
    crate::policy_delivery::PolicyPlaceholderMaskingRequest::from_authorized_field_paths(fields)
}

fn phase_three_bundle(
    row_label: &str,
    plan_digest: impl Into<String>,
    seam_digest: impl Into<String>,
    delivery_digest: impl Into<String>,
) -> MilestoneNineCertificationBundle {
    let narrowed = phase_three_test_narrowed_artifact();
    let support_profile =
        crate::policy_execution_seam::runtime_backed_policy_execution_seam_support_profile();
    MilestoneNineCertificationBundle {
        canonical_query_digest: narrowed.canonical_query_digest().to_string(),
        policy_digest: narrowed.policy_digest().to_string(),
        result_digest: digest_parts(&[
            format!("row:{row_label}"),
            format!("narrowed:{}", narrowed.digest()),
            format!("shape:{}", narrowed.narrowed_result_shape_digest()),
        ]),
        tenant_truth_basis_digest: narrowed.tenant_truth_basis_digest().to_string(),
        tenant_schema_basis_digest: narrowed.tenant_schema_basis_digest().to_string(),
        branch_access_digest: narrowed.branch_access_digest().to_string(),
        schema_variant_digest: "phase3-schema-variant-bound-in-phase1".to_string(),
        execution_mode: row_label.to_string(),
        admission_disposition: "phase3-policy-aware-lowered".to_string(),
        policy_cost_posture: narrowed.cost_posture().as_str().to_string(),
        policy_work_budget_digest: narrowed.work_budget().digest_part(),
        authorized_projection_digest: narrowed
            .authorized_projection()
            .identity()
            .as_str()
            .to_string(),
        narrowed_result_shape_digest: narrowed.narrowed_result_shape_digest().to_string(),
        relationship_proof_digest: narrowed
            .relationship_proof()
            .identity()
            .as_str()
            .to_string(),
        validation_report_digest: narrowed.validation_report().digest().to_string(),
        policy_plan_digest: plan_digest.into(),
        policy_execution_seam_digest: seam_digest.into(),
        delivery_digest: delivery_digest.into(),
        employee_fixture_digest: "phase3-employee-fixture-deferred".to_string(),
        policy_scale_counter_slope_digest: "phase3-policy-scale-deferred".to_string(),
        live_drift_evidence_digest: "phase3-live-drift-deferred".to_string(),
        delivery_width_class_digest: "phase3-delivery-width-bound".to_string(),
        composition_policy_parity_digest: "phase3-composition-parity-deferred".to_string(),
        view_shape_policy_parity_digest: "phase3-view-shape-parity-deferred".to_string(),
        placeholder_denial_digest: "phase3-placeholder-denial-deferred".to_string(),
        counter_snapshot_digest: narrowed
            .validation_report()
            .counter_snapshot_digest()
            .to_string(),
        support_profile_digest: support_profile.profile_digest().to_string(),
    }
}

fn policy_execution_handoff_bundle() -> MilestoneNineCertificationBundle {
    let narrowed = phase_three_test_narrowed_artifact();
    let support_profile =
        crate::policy_execution_seam::runtime_backed_policy_execution_seam_support_profile();
    let handoff =
        crate::policy_execution_seam::runtime_backed_policy_execution_seam_handoff_report();
    MilestoneNineCertificationBundle {
        canonical_query_digest: narrowed.canonical_query_digest().to_string(),
        policy_digest: narrowed.policy_digest().to_string(),
        result_digest: digest_parts(&[
            format!("handoff:{}", handoff.handoff_digest()),
            format!("narrowed:{}", narrowed.digest()),
            format!("shape:{}", narrowed.narrowed_result_shape_digest()),
        ]),
        tenant_truth_basis_digest: narrowed.tenant_truth_basis_digest().to_string(),
        tenant_schema_basis_digest: narrowed.tenant_schema_basis_digest().to_string(),
        branch_access_digest: narrowed.branch_access_digest().to_string(),
        schema_variant_digest: "phase3-schema-variant-bound-in-phase1".to_string(),
        execution_mode: "policy-execution-handoff".to_string(),
        admission_disposition: "runtime-backed-verified-store-and-durable-deferred".to_string(),
        policy_cost_posture: narrowed.cost_posture().as_str().to_string(),
        policy_work_budget_digest: narrowed.work_budget().digest_part(),
        authorized_projection_digest: narrowed
            .authorized_projection()
            .identity()
            .as_str()
            .to_string(),
        narrowed_result_shape_digest: narrowed.narrowed_result_shape_digest().to_string(),
        relationship_proof_digest: narrowed
            .relationship_proof()
            .identity()
            .as_str()
            .to_string(),
        validation_report_digest: narrowed.validation_report().digest().to_string(),
        policy_plan_digest: digest_parts(&[
            format!(
                "m10_handoff:{}",
                handoff.milestone_ten_store_backed_handoff().join("|")
            ),
            format!(
                "m11_handoff:{}",
                handoff.milestone_eleven_durable_handoff().join("|")
            ),
        ]),
        policy_execution_seam_digest: handoff.handoff_digest().to_string(),
        delivery_digest: "durable-delivery-metadata-deferred-to-m11".to_string(),
        employee_fixture_digest: "phase3-handoff-employee-fixture-deferred".to_string(),
        policy_scale_counter_slope_digest: "phase3-handoff-policy-scale-deferred".to_string(),
        live_drift_evidence_digest: "phase3-handoff-live-drift-deferred".to_string(),
        delivery_width_class_digest: "phase3-handoff-delivery-width-deferred".to_string(),
        composition_policy_parity_digest: "phase3-handoff-composition-parity-deferred".to_string(),
        view_shape_policy_parity_digest: "phase3-handoff-view-shape-parity-deferred".to_string(),
        placeholder_denial_digest: "phase3-handoff-placeholder-denial-deferred".to_string(),
        counter_snapshot_digest: digest_parts(&[
            format!(
                "runtime_verified:{}",
                handoff.runtime_backed_verified_surface_count()
            ),
            format!(
                "blocked_or_deferred:{}",
                handoff.blocked_or_deferred_surface_count()
            ),
        ]),
        support_profile_digest: support_profile.profile_digest().to_string(),
    }
}

fn phase_four_bundle(
    row_label: &str,
    employee_fixture_digest: impl Into<String>,
    policy_scale_counter_slope_digest: impl Into<String>,
    live_drift_evidence_digest: impl Into<String>,
    delivery_width_class_digest: impl Into<String>,
    composition_policy_parity_digest: impl Into<String>,
    view_shape_policy_parity_digest: impl Into<String>,
    extra_counter_parts: &[String],
) -> MilestoneNineCertificationBundle {
    let employee_fixture_digest = employee_fixture_digest.into();
    let narrowed = phase_three_test_narrowed_artifact();
    phase_four_bundle_from_narrowed(
        row_label,
        narrowed,
        employee_fixture_digest,
        policy_scale_counter_slope_digest,
        live_drift_evidence_digest,
        delivery_width_class_digest,
        composition_policy_parity_digest,
        view_shape_policy_parity_digest,
        extra_counter_parts,
    )
}

fn phase_four_bundle_from_narrowed(
    row_label: &str,
    narrowed: crate::policy_narrowing::NarrowedPolicyQueryArtifact,
    employee_fixture_digest: impl Into<String>,
    policy_scale_counter_slope_digest: impl Into<String>,
    live_drift_evidence_digest: impl Into<String>,
    delivery_width_class_digest: impl Into<String>,
    composition_policy_parity_digest: impl Into<String>,
    view_shape_policy_parity_digest: impl Into<String>,
    extra_counter_parts: &[String],
) -> MilestoneNineCertificationBundle {
    let employee_fixture_digest = employee_fixture_digest.into();
    let support_profile =
        crate::policy_execution_seam::runtime_backed_policy_execution_seam_support_profile();
    let current_plan = crate::policy_plan::lower_policy_aware_current_plan(&narrowed);
    let scalar_delivery = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &narrowed,
        crate::policy_delivery::DeliveryWidthClass::ScalarDetail,
    )
    .unwrap();
    let placeholder_denial = crate::policy_delivery::deny_policy_placeholder_masking(
        &narrowed,
        policy_placeholder_request([("secret", "salary")]),
    );
    let placeholder_denial_digest = match placeholder_denial {
        Ok(admitted_no_denial) => admitted_no_denial.failure_digest().to_string(),
        Err(error) => digest_parts(&[
            error.failure_class().as_str().to_string(),
            digest_parts(&error.counters().digest_parts()),
        ]),
    };
    MilestoneNineCertificationBundle {
        canonical_query_digest: narrowed.canonical_query_digest().to_string(),
        policy_digest: narrowed.policy_digest().to_string(),
        result_digest: digest_parts(&[
            format!("row:{row_label}"),
            format!("narrowed:{}", narrowed.digest()),
            format!("delivery:{}", scalar_delivery.digest().as_str()),
            format!("employee_fixture:{}", employee_fixture_digest),
        ]),
        tenant_truth_basis_digest: narrowed.tenant_truth_basis_digest().to_string(),
        tenant_schema_basis_digest: narrowed.tenant_schema_basis_digest().to_string(),
        branch_access_digest: narrowed.branch_access_digest().to_string(),
        schema_variant_digest: "phase4-schema-variant-bound-in-fixture".to_string(),
        execution_mode: row_label.to_string(),
        admission_disposition: "phase4-runtime-backed-certified".to_string(),
        policy_cost_posture: narrowed.cost_posture().as_str().to_string(),
        policy_work_budget_digest: narrowed.work_budget().digest_part(),
        authorized_projection_digest: narrowed
            .authorized_projection()
            .identity()
            .as_str()
            .to_string(),
        narrowed_result_shape_digest: narrowed.narrowed_result_shape_digest().to_string(),
        relationship_proof_digest: narrowed
            .relationship_proof()
            .identity()
            .as_str()
            .to_string(),
        validation_report_digest: narrowed.validation_report().digest().to_string(),
        policy_plan_digest: current_plan.core().digest().as_str().to_string(),
        policy_execution_seam_digest: current_plan.core().seam().identity().as_str().to_string(),
        delivery_digest: scalar_delivery.digest().as_str().to_string(),
        employee_fixture_digest,
        policy_scale_counter_slope_digest: policy_scale_counter_slope_digest.into(),
        live_drift_evidence_digest: live_drift_evidence_digest.into(),
        delivery_width_class_digest: delivery_width_class_digest.into(),
        composition_policy_parity_digest: composition_policy_parity_digest.into(),
        view_shape_policy_parity_digest: view_shape_policy_parity_digest.into(),
        placeholder_denial_digest,
        counter_snapshot_digest: {
            let mut counter_parts = vec![
                "phase4_employee_fixture:1".to_string(),
                "phase4_scale_slope:1".to_string(),
                "phase4_live_drift_evidence:1".to_string(),
                "phase4_delivery_width:1".to_string(),
                "phase4_composition_parity:1".to_string(),
            ];
            counter_parts.extend(extra_counter_parts.iter().cloned());
            digest_parts(&counter_parts)
        },
        support_profile_digest: support_profile.profile_digest().to_string(),
    }
}

fn saved_query_reuse_bundle(
    disposition: SavedQueryPolicyReuseDisposition,
) -> MilestoneNineCertificationBundle {
    let support_profile = runtime_backed_policy_tenant_admission_support_profile();
    MilestoneNineCertificationBundle {
        canonical_query_digest: "saved-query-policy-tenant-reuse".to_string(),
        policy_digest: format!("reuse:{}", disposition.as_str()),
        result_digest: digest_parts(&[
            "saved-query-policy-tenant-reuse".to_string(),
            format!("reuse:{}", disposition.as_str()),
        ]),
        tenant_truth_basis_digest: "reuse-tenant-truth".to_string(),
        tenant_schema_basis_digest: "reuse-tenant-schema".to_string(),
        branch_access_digest: "reuse-branch".to_string(),
        schema_variant_digest: "reuse-schema".to_string(),
        execution_mode: PolicyExecutionModeRequest::CurrentRead.as_str().to_string(),
        admission_disposition: PolicyAdmissionDisposition::AdmittedUnchanged
            .as_str()
            .to_string(),
        policy_cost_posture: PolicyCostPosture::ConstantProof.as_str().to_string(),
        policy_work_budget_digest: PolicyWorkBudget::bounded(1, 1, 1).digest_part(),
        authorized_projection_digest: "saved-query-authorized-projection-deferred".to_string(),
        narrowed_result_shape_digest: "saved-query-narrowed-result-shape-deferred".to_string(),
        relationship_proof_digest: "saved-query-relationship-proof-deferred".to_string(),
        validation_report_digest: "saved-query-validation-report-deferred".to_string(),
        policy_plan_digest: "saved-query-policy-plan-deferred".to_string(),
        policy_execution_seam_digest: "saved-query-policy-seam-deferred".to_string(),
        delivery_digest: "saved-query-delivery-deferred".to_string(),
        employee_fixture_digest: "saved-query-employee-fixture-deferred".to_string(),
        policy_scale_counter_slope_digest: "saved-query-policy-scale-deferred".to_string(),
        live_drift_evidence_digest: "saved-query-live-drift-deferred".to_string(),
        delivery_width_class_digest: "saved-query-delivery-width-deferred".to_string(),
        composition_policy_parity_digest: "saved-query-composition-parity-deferred".to_string(),
        view_shape_policy_parity_digest: "saved-query-view-shape-parity-deferred".to_string(),
        placeholder_denial_digest: "saved-query-placeholder-denial-deferred".to_string(),
        counter_snapshot_digest: digest_parts(&[format!("reuse:{}", disposition.as_str())]),
        support_profile_digest: support_profile.profile_digest().to_string(),
    }
}

fn rejection_bundle(
    error: crate::policy_basis::PolicyTenantAdmissionError,
) -> MilestoneNineRejectionBundle {
    let failure_class = match error.failure_class() {
        PolicyTenantAdmissionFailureClass::UnsupportedExecutionMode => {
            MilestoneNineFailureClass::UnsupportedExecutionMode
        }
        PolicyTenantAdmissionFailureClass::BranchAccessDenied => {
            MilestoneNineFailureClass::BranchAccessDenied
        }
        PolicyTenantAdmissionFailureClass::PolicyWorkBudgetDenied => {
            MilestoneNineFailureClass::UnsupportedExecutionMode
        }
        PolicyTenantAdmissionFailureClass::TenantAdmissionDenied => {
            MilestoneNineFailureClass::TenantAdmissionDenied
        }
        _ => MilestoneNineFailureClass::TenantAdmissionDenied,
    };
    MilestoneNineRejectionBundle {
        failure_class,
        failure_digest: digest_parts(&[
            error.failure_class().as_str().to_string(),
            error.message().to_string(),
        ]),
        counter_snapshot_digest: digest_parts(&[
            format!(
                "policy_denials:{}",
                error.counters().policy().policy_basis_denial_count()
            ),
            format!(
                "branch_denials:{}",
                error.counters().policy().branch_access_denial_count()
            ),
            format!(
                "mode_denials:{}",
                error
                    .counters()
                    .policy()
                    .unsupported_execution_mode_denial_count()
            ),
            format!(
                "work_budget_denials:{}",
                error.counters().policy().policy_work_budget_denial_count()
            ),
            format!(
                "hidden_filters:{}",
                error
                    .counters()
                    .tenant()
                    .hidden_tenant_filter_denial_count()
            ),
            format!(
                "schema_fallbacks:{}",
                error
                    .counters()
                    .tenant()
                    .global_schema_fallback_denial_count()
            ),
        ]),
    }
}

fn policy_narrowing_rejection_bundle(
    error: crate::policy_narrowing::PolicyNarrowingError,
) -> MilestoneNineRejectionBundle {
    let failure_class = match error.failure_class() {
        crate::policy_narrowing::PolicyNarrowingFailureClass::RelationshipProofDenied(_) => {
            MilestoneNineFailureClass::RelationshipProofDenied
        }
        _ => MilestoneNineFailureClass::PolicyNarrowingDenied,
    };
    MilestoneNineRejectionBundle {
        failure_class,
        failure_digest: digest_parts(&[
            error.failure_class().as_str().to_string(),
            error.message().to_string(),
        ]),
        counter_snapshot_digest: digest_parts(&error.counters().digest_parts()),
    }
}

fn policy_execution_seam_rejection_bundle(
    error: crate::policy_execution_seam::PolicyAwareExecutionSeamError,
) -> MilestoneNineRejectionBundle {
    MilestoneNineRejectionBundle {
        failure_class: MilestoneNineFailureClass::PolicyExecutionSeamDenied,
        failure_digest: digest_parts(&[
            error.failure_class().as_str().to_string(),
            error.message().to_string(),
        ]),
        counter_snapshot_digest: digest_parts(&error.counters().digest_parts()),
    }
}

fn rejection_for_mode(mode: PolicyExecutionModeRequest) -> MilestoneNineRejectionBundle {
    let canonical = canonical_query();
    let policy = base_policy(false);
    let branch = BranchAccessGrant::synthetic_granted("branch-a", &policy);
    let error =
        admit_policy_tenant_context(canonical.query(), policy, tenant(), branch, schema(), mode)
            .unwrap_err();
    rejection_bundle(error)
}

fn rejection_rows() -> Vec<MilestoneNineRejectionRow> {
    let control = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false);

    let canonical = canonical_query();
    let policy = base_policy(false);
    let branch_denial = admit_policy_tenant_context(
        canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_denied("branch-a", "no_relationship_path", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap_err();
    let unknown_cost_policy = PolicyRuleSnapshot::synthetic_authority_with_budget(
        "runtime-policy",
        "rules-v1",
        PolicyEpoch::Synthetic(7),
        true,
        PolicyCostPosture::UnknownCost,
        Some(PolicyWorkBudget::bounded(1, 1, 1)),
    );
    let unknown_cost = admit_policy_tenant_context(
        canonical.query(),
        unknown_cost_policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &unknown_cost_policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap_err();
    let hidden_tenant = admit_policy_tenant_context(
        canonical.query(),
        policy.clone(),
        TenantBindingSnapshot::synthetic_hidden_filter(
            "tenant-a",
            "branch-a",
            "schema-a",
            TenantBasisEpoch::Synthetic(3),
        ),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap_err();
    let global_fallback = admit_policy_tenant_context(
        canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        SchemaVariantSnapshot::synthetic_global_fallback("tenant-a", "schema-a"),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap_err();
    let drift = SavedQueryPolicyReuseDescriptor::new(
        "saved-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::CurrentRead,
        "policy-b",
        "tenant-truth-b",
        "tenant-schema-b",
        "branch-b",
        PolicyExecutionModeRequest::CurrentRead,
    );
    let drift_class = classify_saved_query_policy_tenant_reuse(&drift);
    let phase_two_canonical = canonical_query_with_secret_projection();
    let phase_two_admitted = admit_policy_tenant_context(
        phase_two_canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    let masked_predicate_canonical = canonical_query_with_secret_predicate();
    let masked_predicate_admitted = admit_policy_tenant_context(
        masked_predicate_canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    let masked_predicate = narrow_policy_query(
        &masked_predicate_canonical,
        masked_predicate_admitted.clone(),
        phase_two_mask_snapshot(
            &masked_predicate_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let masked_ordering_canonical = canonical_query_with_secret_ordering();
    let masked_ordering_admitted = admit_policy_tenant_context(
        masked_ordering_canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    let masked_ordering = narrow_policy_query(
        &masked_ordering_canonical,
        masked_ordering_admitted.clone(),
        phase_two_mask_snapshot(
            &masked_ordering_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_non_disclosing_use_only(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let masked_grouping = narrow_policy_query(
        &phase_two_canonical,
        phase_two_admitted.clone(),
        phase_two_mask_snapshot(
            &phase_two_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_non_disclosing_use_only(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none().with_grouping_field(
            crate::authoring::AspectFieldKey::from_authoring_parts("secret", "salary").unwrap(),
        ),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let template_hidden_influence = narrow_policy_query(
        &phase_two_canonical,
        phase_two_admitted.clone(),
        phase_two_mask_snapshot(
            &phase_two_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none().with_template_predicate_field(
            crate::authoring::AspectFieldKey::from_authoring_parts("secret", "salary").unwrap(),
        ),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let host_callback = narrow_policy_query(
        &phase_two_canonical,
        phase_two_admitted.clone(),
        phase_two_mask_snapshot(
            &phase_two_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::new(
            vec![RelationshipProofDescriptor::host_callback_forbidden(
                "authz",
            )],
            RelationshipProofBudget::bounded(1, 1),
        ),
    )
    .unwrap_err();
    let query_conflict = narrow_policy_query(
        &phase_two_canonical,
        phase_two_admitted.clone(),
        phase_two_mask_snapshot(
            &phase_two_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::new(
            vec![RelationshipProofDescriptor::query_shape_mismatch_for_test(
                "different-query-digest",
            )],
            RelationshipProofBudget::bounded(1, 1),
        ),
    )
    .unwrap_err();
    let unbounded_recursion = narrow_policy_query(
        &phase_two_canonical,
        phase_two_admitted.clone(),
        phase_two_mask_snapshot(
            &phase_two_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::new(
            vec![RelationshipProofDescriptor::unbounded_recursive_walk_for_test("manager")],
            RelationshipProofBudget::bounded(1, 1),
        ),
    )
    .unwrap_err();
    let saved_narrowing_drift =
        classify_saved_policy_narrowing_reuse(&SavedPolicyNarrowingReuseDescriptor::new(
            "saved-a",
            "narrowed-a",
            "policy-a",
            "tenant-truth-a",
            "tenant-schema-a",
            "projection-a",
            "proof-a",
            "policy-b",
            "tenant-truth-a",
            "tenant-schema-a",
            "projection-a",
            "proof-a",
        ));
    let phase_three_narrowed = phase_three_test_narrowed_artifact();
    let raw_branch_bypass = crate::policy_plan::lower_policy_aware_branch_plan(
        &phase_three_narrowed,
        crate::policy_plan::PolicyAwareReadBasis::admitted_branch(
            "wrong-branch-digest",
            "phase3-branch-basis",
        ),
    )
    .unwrap_err();
    let raw_diff_scrub = crate::policy_plan::deny_raw_diff_scrub();
    let masked_live_relevance = crate::policy_live::admit_policy_aware_live_plan(
        &phase_three_narrowed,
        &[authorized_projection_field("secret", "salary")],
        crate::policy_live::PolicyDriftDisposition::NoChange,
        crate::policy_live::PolicyLiveDensityPosture::SparseDelta,
    )
    .unwrap_err();
    let delivery_overexposure = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &phase_three_narrowed,
        crate::policy_delivery::DeliveryWidthClass::DeniedWidthInflation,
    )
    .unwrap_err();
    let placeholder_masking = crate::policy_delivery::deny_policy_placeholder_masking(
        &phase_three_narrowed,
        policy_placeholder_request([("secret", "salary")]),
    )
    .unwrap_err();
    let store_deferred = crate::policy_plan::lower_policy_aware_historical_plan(
        &phase_three_narrowed,
        crate::policy_plan::PolicyAwareHistoricalBasis::store_backed_deferred("phase3-store-basis"),
    )
    .unwrap_err();
    let masked_aggregation_influence = narrow_policy_query(
        &phase_two_canonical,
        phase_two_admitted.clone(),
        phase_two_mask_snapshot(
            &phase_two_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none().with_aggregation_field(
            crate::authoring::AspectFieldKey::from_authoring_parts("secret", "salary").unwrap(),
        ),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let masked_cursor_influence = narrow_policy_query(
        &phase_two_canonical,
        phase_two_admitted.clone(),
        phase_two_mask_snapshot(
            &phase_two_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none().with_cursor_field(
            crate::authoring::AspectFieldKey::from_authoring_parts("secret", "salary").unwrap(),
        ),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let masked_view_membership_influence = narrow_policy_query(
        &phase_two_canonical,
        phase_two_admitted.clone(),
        phase_two_mask_snapshot(
            &phase_two_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none().with_view_membership_field(
            crate::authoring::AspectFieldKey::from_authoring_parts("secret", "salary").unwrap(),
        ),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let durable_cursor_deferred = crate::policy_execution_seam::deny_durable_policy_cursor_claim();
    let durable_artifact_reload_deferred =
        crate::policy_execution_seam::deny_durable_policy_artifact_reload_claim();
    let durable_delivery_metadata_deferred =
        crate::policy_execution_seam::deny_durable_policy_delivery_metadata_reload_claim();
    let per_row_allocation = crate::policy_execution_seam::deny_policy_per_row_allocation_claim();
    let cross_tenant_fanout = crate::policy_execution_seam::deny_policy_cross_tenant_fanout_claim();
    let saved_query_bypass = crate::policy_execution_seam::deny_saved_query_policy_bypass_claim();
    let unsupported_workflow_composition =
        crate::policy_execution_seam::deny_unsupported_policy_workflow_composition_claim();

    vec![
        MilestoneNineRejectionRow {
            row_name: "live-subscription-deferred-before-truth",
            perturbation_class: MilestoneNinePerturbationClass::LiveSubscriptionDeferred,
            control_lane: control.clone(),
            hostile_lane: rejection_for_mode(PolicyExecutionModeRequest::LiveSubscription),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "historical-diff-deferred-before-truth",
            perturbation_class: MilestoneNinePerturbationClass::HistoricalDiffDeferred,
            control_lane: control.clone(),
            hostile_lane: rejection_for_mode(PolicyExecutionModeRequest::HistoricalDiff),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "unknown-policy-cost-denied-before-truth",
            perturbation_class: MilestoneNinePerturbationClass::UnknownPolicyCostDenied,
            control_lane: control.clone(),
            hostile_lane: rejection_bundle(unknown_cost.clone()),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "branch-denial-before-tenant-truth",
            perturbation_class: MilestoneNinePerturbationClass::BranchDeniedBeforeTruth,
            control_lane: control.clone(),
            hostile_lane: rejection_bundle(branch_denial),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "hidden-tenant-filter-denied",
            perturbation_class: MilestoneNinePerturbationClass::HiddenTenantFilterDenied,
            control_lane: control.clone(),
            hostile_lane: rejection_bundle(hidden_tenant),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "global-schema-fallback-denied",
            perturbation_class: MilestoneNinePerturbationClass::GlobalSchemaFallbackDenied,
            control_lane: control.clone(),
            hostile_lane: rejection_bundle(global_fallback),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "saved-query-policy-tenant-drift",
            perturbation_class: MilestoneNinePerturbationClass::SavedQueryPolicyTenantDrift,
            control_lane: saved_query_reuse_bundle(
                SavedQueryPolicyReuseDisposition::LegalNoSemanticChange,
            ),
            hostile_lane: MilestoneNineRejectionBundle {
                failure_class: MilestoneNineFailureClass::SavedQueryPolicyTenantDrift,
                failure_digest: digest_parts(&[drift_class.as_str().to_string()]),
                counter_snapshot_digest: digest_parts(&[format!("reuse:{}", drift_class.as_str())]),
            },
            parity_lane: saved_query_reuse_bundle(
                SavedQueryPolicyReuseDisposition::LegalNoSemanticChange,
            ),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-predicate-denies-before-narrowing",
            perturbation_class:
                MilestoneNinePerturbationClass::MaskedPredicateDeniedBeforeNarrowing,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(masked_predicate),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-ordering-denies-before-narrowing",
            perturbation_class: MilestoneNinePerturbationClass::MaskedOrderingDeniedBeforeNarrowing,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(masked_ordering),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-grouping-denies-before-narrowing",
            perturbation_class: MilestoneNinePerturbationClass::MaskedGroupingDeniedBeforeNarrowing,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(masked_grouping),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "relationship-proof-host-callback-forbidden",
            perturbation_class:
                MilestoneNinePerturbationClass::RelationshipProofHostCallbackForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(host_callback),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "relationship-proof-unbounded-recursion-denied",
            perturbation_class:
                MilestoneNinePerturbationClass::RelationshipProofUnboundedRecursionDenied,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(unbounded_recursion.clone()),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "relationship-proof-query-conflict-denied",
            perturbation_class:
                MilestoneNinePerturbationClass::RelationshipProofQueryConflictDenied,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(query_conflict),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "template-hidden-influence-denied",
            perturbation_class: MilestoneNinePerturbationClass::TemplateHiddenInfluenceDenied,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(template_hidden_influence),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "saved-query-policy-drift-renarrowing-required",
            perturbation_class:
                MilestoneNinePerturbationClass::SavedQueryPolicyDriftRenarrowingRequired,
            control_lane: control.clone(),
            hostile_lane: MilestoneNineRejectionBundle {
                failure_class: MilestoneNineFailureClass::SavedQueryPolicyTenantDrift,
                failure_digest: digest_parts(&[saved_narrowing_drift.as_str().to_string()]),
                counter_snapshot_digest: digest_parts(&[format!(
                    "saved_narrowing_reuse:{}",
                    saved_narrowing_drift.as_str()
                )]),
            },
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "unknown-narrowing-cost-denied-before-truth",
            perturbation_class: MilestoneNinePerturbationClass::UnknownNarrowingCostDenied,
            control_lane: control.clone(),
            hostile_lane: rejection_bundle(unknown_cost),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "phase-two-no-truth-touch",
            perturbation_class: MilestoneNinePerturbationClass::PhaseTwoNoTruthTouch,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(unbounded_recursion),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "raw-current-plan-bypass-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::RawCurrentPlanBypassForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(raw_branch_bypass.clone()),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "raw-branch-plan-bypass-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::RawBranchPlanBypassForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(raw_branch_bypass),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "raw-historical-plan-bypass-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::RawHistoricalPlanBypassForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(store_deferred.clone()),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "raw-diff-scrub-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::RawDiffScrubForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(raw_diff_scrub),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-live-relevance-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::MaskedLiveRelevanceForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(masked_live_relevance),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "delivery-shape-overexposure-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::DeliveryShapeOverexposureForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(delivery_overexposure),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "store-backed-policy-execution-deferred",
            perturbation_class: MilestoneNinePerturbationClass::StoreBackedPolicyExecutionDeferred,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(store_deferred),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "durable-policy-cursor-deferred",
            perturbation_class: MilestoneNinePerturbationClass::DurablePolicyCursorDeferred,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(durable_cursor_deferred),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "durable-policy-artifact-reload-deferred",
            perturbation_class: MilestoneNinePerturbationClass::DurablePolicyArtifactReloadDeferred,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(durable_artifact_reload_deferred),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "durable-policy-delivery-metadata-deferred",
            perturbation_class:
                MilestoneNinePerturbationClass::DurablePolicyDeliveryMetadataDeferred,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(
                durable_delivery_metadata_deferred,
            ),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "phase-three-no-truth-touch-before-plan-admission",
            perturbation_class: MilestoneNinePerturbationClass::PhaseThreeNoTruthTouch,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(
                crate::policy_plan::deny_raw_diff_scrub(),
            ),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-placeholder-shape-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::MaskedPlaceholderShapeForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(placeholder_masking),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-aggregation-without-witness-forbidden",
            perturbation_class:
                MilestoneNinePerturbationClass::MaskedAggregationWithoutWitnessForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(masked_aggregation_influence),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-cursor-without-witness-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::MaskedCursorWithoutWitnessForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(masked_cursor_influence),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-view-membership-without-witness-forbidden",
            perturbation_class:
                MilestoneNinePerturbationClass::MaskedViewMembershipWithoutWitnessForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(masked_view_membership_influence),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "policy-per-row-allocation-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::PolicyPerRowAllocationForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(per_row_allocation),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "policy-cross-tenant-fanout-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::PolicyCrossTenantFanoutForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(cross_tenant_fanout),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "saved-query-policy-bypass-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::SavedQueryPolicyBypassForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(saved_query_bypass),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "unsupported-policy-workflow-composition-forbidden",
            perturbation_class:
                MilestoneNinePerturbationClass::UnsupportedPolicyWorkflowCompositionForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(unsupported_workflow_composition),
            parity_lane: control,
        },
    ]
}

fn canonical_rows() -> Vec<MilestoneNineCertificationRow> {
    let current = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false);
    let branch = admitted_bundle(PolicyExecutionModeRequest::BranchRead, false);
    let historical = admitted_bundle(PolicyExecutionModeRequest::HistoricalRead, false);
    let narrowed = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, true);
    let bounded = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false);
    let exact_reuse =
        saved_query_reuse_bundle(SavedQueryPolicyReuseDisposition::LegalNoSemanticChange);
    let support = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false);
    let phase_two_canonical = canonical_query_with_secret_projection();
    let policy = base_policy(true);
    let phase_two_admitted = admit_policy_tenant_context(
        phase_two_canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    let phase_two_mask = crate::authorized_projection::PolicyAspectMask::allow_all()
        .with_masked(secret_salary_key());
    let phase_two_no_proof = phase_two_bundle(
        phase_two_canonical.clone(),
        phase_two_mask.clone(),
        RelationshipProofDescriptorSet::none(),
    );
    let phase_two_unmasked = phase_two_bundle(
        phase_two_canonical.clone(),
        crate::authorized_projection::PolicyAspectMask::allow_all(),
        RelationshipProofDescriptorSet::none(),
    );
    let non_disclosing_use = phase_two_bundle(
        phase_two_canonical.clone(),
        crate::authorized_projection::PolicyAspectMask::allow_all()
            .with_non_disclosing_use_only(secret_salary_key()),
        RelationshipProofDescriptorSet::none(),
    );
    let phase_two_direct_proof = phase_two_bundle(
        canonical_query_with_manager_traversal(),
        phase_two_mask.clone(),
        RelationshipProofDescriptorSet::new(
            vec![RelationshipProofDescriptor::direct_edge(
                "manager",
                phase_two_admitted.bundle().policy_digest(),
            )],
            RelationshipProofBudget::bounded(1, 1),
        ),
    );
    let phase_two_tenant_membership = phase_two_bundle(
        phase_two_canonical.clone(),
        phase_two_mask.clone(),
        RelationshipProofDescriptorSet::new(
            vec![RelationshipProofDescriptor::tenant_membership(
                phase_two_admitted.bundle().tenant_schema_basis_digest(),
            )],
            RelationshipProofBudget::bounded(1, 1),
        ),
    );
    let saved_exact =
        classify_saved_policy_narrowing_reuse(&SavedPolicyNarrowingReuseDescriptor::new(
            "saved-a",
            phase_two_no_proof.authorized_projection_digest.clone(),
            phase_two_no_proof.policy_digest.clone(),
            phase_two_no_proof.tenant_truth_basis_digest.clone(),
            phase_two_no_proof.tenant_schema_basis_digest.clone(),
            phase_two_no_proof.authorized_projection_digest.clone(),
            phase_two_no_proof.relationship_proof_digest.clone(),
            phase_two_no_proof.policy_digest.clone(),
            phase_two_no_proof.tenant_truth_basis_digest.clone(),
            phase_two_no_proof.tenant_schema_basis_digest.clone(),
            phase_two_no_proof.authorized_projection_digest.clone(),
            phase_two_no_proof.relationship_proof_digest.clone(),
        ));
    let mut saved_exact_bundle = phase_two_no_proof.clone();
    saved_exact_bundle.counter_snapshot_digest =
        digest_parts(&[format!("saved_narrowing_reuse:{}", saved_exact.as_str())]);
    let phase_three_narrowed = phase_three_test_narrowed_artifact();
    let current_plan = crate::policy_plan::lower_policy_aware_current_plan(&phase_three_narrowed);
    let branch_plan = crate::policy_plan::lower_policy_aware_branch_plan(
        &phase_three_narrowed,
        crate::policy_plan::PolicyAwareReadBasis::admitted_branch(
            phase_three_narrowed.branch_access_digest(),
            "phase3-branch-basis",
        ),
    )
    .unwrap();
    let historical_plan = crate::policy_plan::lower_policy_aware_historical_plan(
        &phase_three_narrowed,
        crate::policy_plan::PolicyAwareHistoricalBasis::runtime_backed("phase3-historical-basis"),
    )
    .unwrap();
    let diff_plan = crate::policy_plan::lower_policy_aware_diff_plan(
        &phase_three_narrowed,
        crate::policy_plan::PolicyAwareDiffBasisPair::runtime_backed(
            "phase3-left-basis",
            "phase3-right-basis",
        ),
    )
    .unwrap();
    let live_plan = crate::policy_live::admit_policy_aware_live_plan(
        &phase_three_narrowed,
        &native_authorized_projection_fields(&phase_three_narrowed),
        crate::policy_live::PolicyDriftDisposition::NoChange,
        crate::policy_live::PolicyLiveDensityPosture::SparseDelta,
    )
    .unwrap();
    let delivery_shape = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &phase_three_narrowed,
        crate::policy_delivery::DeliveryWidthClass::ScalarDetail,
    )
    .unwrap();
    let optimizer_input =
        crate::policy_plan::lower_policy_aware_optimizer_input(&phase_three_narrowed);
    let phase_three_current = phase_three_bundle(
        "policy-aware-current",
        current_plan.core().digest().as_str(),
        current_plan.core().seam().identity().as_str(),
        "phase3-current-delivery-not-lowered",
    );
    let phase_three_branch = phase_three_bundle(
        "policy-aware-branch",
        branch_plan.core().digest().as_str(),
        branch_plan.core().seam().identity().as_str(),
        "phase3-branch-delivery-not-lowered",
    );
    let phase_three_historical = phase_three_bundle(
        "policy-aware-historical",
        historical_plan.core().digest().as_str(),
        historical_plan.core().seam().identity().as_str(),
        "phase3-historical-delivery-not-lowered",
    );
    let phase_three_diff = phase_three_bundle(
        "policy-aware-diff",
        diff_plan.core().digest().as_str(),
        diff_plan.core().seam().identity().as_str(),
        "phase3-diff-delivery-not-lowered",
    );
    let phase_three_live = phase_three_bundle(
        "policy-aware-live",
        live_plan.core().digest().as_str(),
        live_plan.core().seam().identity().as_str(),
        "phase3-live-delivery-not-lowered",
    );
    let phase_three_delivery = phase_three_bundle(
        "policy-aware-delivery",
        delivery_shape.seam().identity().as_str(),
        delivery_shape.seam().identity().as_str(),
        delivery_shape.digest().as_str(),
    );
    let phase_three_optimizer = phase_three_bundle(
        "policy-aware-optimizer",
        optimizer_input.optimizer_input_digest(),
        "phase3-optimizer-seam-bound-to-narrowed",
        "phase3-optimizer-delivery-not-lowered",
    );
    let phase_three_handoff = policy_execution_handoff_bundle();
    let employee_fixture = crate::policy_certification::employee_record_policy_fixture();
    let employee_alpha = employee_fixture.certify(
        crate::policy_certification::EmployeeRecordPolicyScenario::new(
            crate::policy_certification::EmployeeRecordTenantVariant::TenantAlpha,
            crate::policy_certification::EmployeeRecordQueryFamily::DirectDetail,
        ),
    );
    let employee_beta = employee_fixture.certify(
        crate::policy_certification::EmployeeRecordPolicyScenario::new(
            crate::policy_certification::EmployeeRecordTenantVariant::TenantBeta,
            crate::policy_certification::EmployeeRecordQueryFamily::DirectDetail,
        ),
    );
    let scale_report = crate::policy_certification::employee_record_policy_scale_report();
    let live_drift_readmission_plan = crate::policy_live::admit_policy_aware_live_plan(
        &phase_three_narrowed,
        &native_authorized_projection_fields(&phase_three_narrowed),
        crate::policy_live::PolicyDriftDisposition::FreshAdmissionFromCheckpoint,
        crate::policy_live::PolicyLiveDensityPosture::SparseDelta,
    )
    .unwrap();
    let live_drift_readmission = crate::policy_live::certify_policy_live_drift_evidence(
        &live_drift_readmission_plan,
        crate::policy_live::PolicyLiveEpochEvidence::new(
            "previous-policy-digest",
            phase_three_narrowed.tenant_truth_basis_digest(),
            phase_three_narrowed.policy_digest(),
            phase_three_narrowed.tenant_truth_basis_digest(),
        ),
        crate::policy_live::PolicyLiveDensityEvidence::new(
            phase_three_narrowed
                .authorized_projection()
                .visible_field_paths()
                .len(),
            1,
            1,
        ),
    )
    .unwrap();
    let live_burst_readmission_plan = crate::policy_live::admit_policy_aware_live_plan(
        &phase_three_narrowed,
        &native_authorized_projection_fields(&phase_three_narrowed),
        crate::policy_live::PolicyDriftDisposition::NoChange,
        crate::policy_live::PolicyLiveDensityPosture::BurstReadmission,
    )
    .unwrap();
    let live_density_honesty = crate::policy_live::certify_policy_live_drift_evidence(
        &live_burst_readmission_plan,
        crate::policy_live::PolicyLiveEpochEvidence::new(
            phase_three_narrowed.policy_digest(),
            phase_three_narrowed.tenant_truth_basis_digest(),
            phase_three_narrowed.policy_digest(),
            phase_three_narrowed.tenant_truth_basis_digest(),
        ),
        crate::policy_live::PolicyLiveDensityEvidence::new(
            phase_three_narrowed
                .authorized_projection()
                .visible_field_paths()
                .len(),
            phase_three_narrowed
                .authorized_projection()
                .visible_field_paths()
                .len(),
            1,
        ),
    )
    .unwrap();
    let narrow_delivery = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &phase_three_narrowed,
        crate::policy_delivery::DeliveryWidthClass::NarrowCollection,
    )
    .unwrap();
    let grouped_delivery = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &phase_three_narrowed,
        crate::policy_delivery::DeliveryWidthClass::GroupedDelta,
    )
    .unwrap();
    let diff_delivery = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &phase_three_narrowed,
        crate::policy_delivery::DeliveryWidthClass::DiffDelta,
    )
    .unwrap();
    let delivery_width_digest = digest_parts(&[
        format!("scalar:{}", delivery_shape.report().digest()),
        format!("narrow:{}", narrow_delivery.report().digest()),
        format!("grouped:{}", grouped_delivery.report().digest()),
        format!("diff:{}", diff_delivery.report().digest()),
    ]);
    let composition_parity = crate::policy_certification::policy_composition_parity_report(
        phase_three_narrowed.digest(),
    );
    let mask_parity = crate::policy_certification::policy_mask_parity_report(
        phase_two_unmasked.authorized_projection_digest.clone(),
        phase_two_no_proof.authorized_projection_digest.clone(),
        phase_two_no_proof.narrowed_result_shape_digest.clone(),
        employee_alpha.masked_field_digest(),
    );
    let view_shape_parity = crate::policy_certification::policy_view_shape_parity_report(
        delivery_shape.digest().as_str(),
        grouped_delivery.digest().as_str(),
        "identity-aware-inspector-delivery-preserves-classification",
    );
    let identity_inspector_parity =
        crate::policy_certification::policy_identity_aware_inspector_parity_report(
            "milestone-seven-identity-classification-preserved",
            "identity-aware-inspector-delivery-preserves-classification",
            phase_two_no_proof.narrowed_result_shape_digest.clone(),
        );
    let phase_four_employee = phase_four_bundle(
        "employee-record-fixture",
        employee_alpha.employee_fixture_digest(),
        scale_report.digest().as_str(),
        live_density_honesty.digest(),
        delivery_width_digest.clone(),
        composition_parity.parity_digest(),
        view_shape_parity.parity_digest(),
        &[],
    );
    let phase_four_tenant_beta = phase_four_bundle(
        "employee-record-tenant-beta",
        employee_beta.employee_fixture_digest(),
        scale_report.digest().as_str(),
        live_density_honesty.digest(),
        delivery_width_digest.clone(),
        composition_parity.parity_digest(),
        view_shape_parity.parity_digest(),
        &[],
    );
    let phase_four_delivery_width = phase_four_bundle(
        "delivery-width-class-honesty",
        employee_alpha.employee_fixture_digest(),
        scale_report.digest().as_str(),
        live_density_honesty.digest(),
        delivery_width_digest.clone(),
        composition_parity.parity_digest(),
        view_shape_parity.parity_digest(),
        &[],
    );
    let phase_four_mask_parity = phase_four_bundle(
        "masked-versus-unmasked-policy-parity",
        employee_alpha.employee_fixture_digest(),
        scale_report.digest().as_str(),
        live_density_honesty.digest(),
        delivery_width_digest.clone(),
        mask_parity.parity_digest(),
        view_shape_parity.parity_digest(),
        &[],
    );
    let phase_four_unmasked_policy = phase_four_bundle_from_narrowed(
        "masked-versus-unmasked-policy-control",
        phase_three_test_unmasked_artifact(),
        employee_alpha.employee_fixture_digest(),
        scale_report.digest().as_str(),
        live_density_honesty.digest(),
        delivery_width_digest.clone(),
        mask_parity.parity_digest(),
        view_shape_parity.parity_digest(),
        &[],
    );
    let phase_four_live_drift = phase_four_bundle(
        "live-policy-epoch-drift-readmission",
        employee_alpha.employee_fixture_digest(),
        scale_report.digest().as_str(),
        live_drift_readmission.digest(),
        delivery_width_digest.clone(),
        composition_parity.parity_digest(),
        view_shape_parity.parity_digest(),
        &live_drift_readmission.counters().digest_parts(),
    );
    let phase_four_live_density = phase_four_bundle(
        "live-policy-density-posture-honesty",
        employee_alpha.employee_fixture_digest(),
        scale_report.digest().as_str(),
        live_density_honesty.digest(),
        delivery_width_digest.clone(),
        composition_parity.parity_digest(),
        view_shape_parity.parity_digest(),
        &live_density_honesty.counters().digest_parts(),
    );
    let phase_four_scale = phase_four_bundle(
        "policy-scale-slope",
        employee_alpha.employee_fixture_digest(),
        scale_report.digest().as_str(),
        live_density_honesty.digest(),
        delivery_width_digest.clone(),
        composition_parity.parity_digest(),
        view_shape_parity.parity_digest(),
        &[],
    );
    let phase_four_composition = phase_four_bundle(
        "policy-composition-parity",
        employee_alpha.employee_fixture_digest(),
        scale_report.digest().as_str(),
        live_density_honesty.digest(),
        delivery_width_digest.clone(),
        composition_parity.parity_digest(),
        view_shape_parity.parity_digest(),
        &[],
    );
    let phase_four_view_shape = phase_four_bundle(
        "policy-view-shape-parity",
        employee_alpha.employee_fixture_digest(),
        scale_report.digest().as_str(),
        live_density_honesty.digest(),
        delivery_width_digest.clone(),
        composition_parity.parity_digest(),
        view_shape_parity.parity_digest(),
        &[],
    );
    let phase_four_identity_inspector = phase_four_bundle(
        "policy-identity-aware-inspector-parity",
        employee_alpha.employee_fixture_digest(),
        scale_report.digest().as_str(),
        live_density_honesty.digest(),
        delivery_width_digest,
        composition_parity.parity_digest(),
        identity_inspector_parity.parity_digest(),
        &[],
    );

    vec![
        MilestoneNineCertificationRow {
            row_name: "current-read-policy-tenant-admission",
            perturbation_class: MilestoneNinePerturbationClass::CurrentReadAdmission,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: current.clone(),
            hostile_lane: current.clone(),
            parity_lane: current,
        },
        MilestoneNineCertificationRow {
            row_name: "branch-read-policy-tenant-admission",
            perturbation_class: MilestoneNinePerturbationClass::BranchReadAdmission,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false),
            hostile_lane: branch.clone(),
            parity_lane: branch,
        },
        MilestoneNineCertificationRow {
            row_name: "historical-read-policy-tenant-admission",
            perturbation_class: MilestoneNinePerturbationClass::HistoricalReadAdmission,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false),
            hostile_lane: historical.clone(),
            parity_lane: historical,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-narrowing-disposition",
            perturbation_class: MilestoneNinePerturbationClass::PolicyNarrowingDisposition,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false),
            hostile_lane: narrowed.clone(),
            parity_lane: narrowed,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-work-budget-explicitness",
            perturbation_class: MilestoneNinePerturbationClass::PolicyWorkBudgetExplicitness,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: bounded.clone(),
            hostile_lane: bounded.clone(),
            parity_lane: bounded,
        },
        MilestoneNineCertificationRow {
            row_name: "saved-query-exact-policy-tenant-reuse",
            perturbation_class: MilestoneNinePerturbationClass::SavedQueryExactReuse,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: exact_reuse.clone(),
            hostile_lane: exact_reuse.clone(),
            parity_lane: exact_reuse,
        },
        MilestoneNineCertificationRow {
            row_name: "support-profile-honesty",
            perturbation_class: MilestoneNinePerturbationClass::SupportProfileHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: support.clone(),
            hostile_lane: support.clone(),
            parity_lane: support,
        },
        MilestoneNineCertificationRow {
            row_name: "authorized-projection-removes-masked-aspect",
            perturbation_class:
                MilestoneNinePerturbationClass::AuthorizedProjectionRemovesMaskedAspect,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false),
            hostile_lane: phase_two_no_proof.clone(),
            parity_lane: phase_two_no_proof.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "non-disclosing-use-is-not-delivered",
            perturbation_class: MilestoneNinePerturbationClass::NonDisclosingUseIsNotDelivered,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_two_no_proof.clone(),
            hostile_lane: non_disclosing_use.clone(),
            parity_lane: non_disclosing_use,
        },
        MilestoneNineCertificationRow {
            row_name: "relationship-proof-direct-edge-admission",
            perturbation_class:
                MilestoneNinePerturbationClass::RelationshipProofDirectEdgeAdmission,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_two_no_proof.clone(),
            hostile_lane: phase_two_direct_proof.clone(),
            parity_lane: phase_two_direct_proof.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "relationship-proof-tenant-membership-admission",
            perturbation_class:
                MilestoneNinePerturbationClass::RelationshipProofTenantMembershipAdmission,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_two_no_proof.clone(),
            hostile_lane: phase_two_tenant_membership.clone(),
            parity_lane: phase_two_tenant_membership,
        },
        MilestoneNineCertificationRow {
            row_name: "narrowed-artifact-binds-policy-tenant-schema",
            perturbation_class:
                MilestoneNinePerturbationClass::NarrowedArtifactBindsPolicyTenantSchema,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_two_no_proof.clone(),
            hostile_lane: phase_two_no_proof.clone(),
            parity_lane: phase_two_no_proof.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "saved-query-exact-reuse-narrows-identically",
            perturbation_class:
                MilestoneNinePerturbationClass::SavedQueryExactReuseNarrowsIdentically,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: saved_exact_bundle.clone(),
            hostile_lane: saved_exact_bundle.clone(),
            parity_lane: saved_exact_bundle,
        },
        MilestoneNineCertificationRow {
            row_name: "optimizer-input-excludes-masked-fields",
            perturbation_class: MilestoneNinePerturbationClass::OptimizerInputExcludesMaskedFields,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_two_no_proof.clone(),
            hostile_lane: phase_two_no_proof.clone(),
            parity_lane: phase_two_no_proof.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "phase-two-support-profile-honesty",
            perturbation_class: MilestoneNinePerturbationClass::PhaseTwoSupportProfileHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_two_no_proof.clone(),
            hostile_lane: phase_two_no_proof.clone(),
            parity_lane: phase_two_no_proof,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-current-plan-lowering",
            perturbation_class: MilestoneNinePerturbationClass::PolicyAwareCurrentPlanLowering,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_current.clone(),
            parity_lane: phase_three_current.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-branch-plan-lowering",
            perturbation_class: MilestoneNinePerturbationClass::PolicyAwareBranchPlanLowering,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_branch.clone(),
            parity_lane: phase_three_branch,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-historical-plan-runtime-backed-lowering",
            perturbation_class: MilestoneNinePerturbationClass::PolicyAwareHistoricalPlanLowering,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_historical.clone(),
            parity_lane: phase_three_historical,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-diff-plan-runtime-backed-lowering",
            perturbation_class: MilestoneNinePerturbationClass::PolicyAwareDiffPlanLowering,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_diff.clone(),
            parity_lane: phase_three_diff,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-live-admission",
            perturbation_class: MilestoneNinePerturbationClass::PolicyAwareLiveAdmission,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_live.clone(),
            parity_lane: phase_three_live,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-delivery-shape-derived-after-mask",
            perturbation_class:
                MilestoneNinePerturbationClass::PolicyAwareDeliveryShapeDerivedAfterMask,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_delivery.clone(),
            parity_lane: phase_three_delivery,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-optimizer-input-only",
            perturbation_class: MilestoneNinePerturbationClass::PolicyAwareOptimizerInputOnly,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_optimizer.clone(),
            parity_lane: phase_three_optimizer,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-execution-seam-parity",
            perturbation_class: MilestoneNinePerturbationClass::PolicyExecutionSeamParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_current.clone(),
            parity_lane: phase_three_current.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "policy-execution-handoff-honesty",
            perturbation_class: MilestoneNinePerturbationClass::PolicyExecutionHandoffHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_three_handoff.clone(),
            hostile_lane: phase_three_handoff.clone(),
            parity_lane: phase_three_handoff,
        },
        MilestoneNineCertificationRow {
            row_name: "employee-record-fixture-policy-basis",
            perturbation_class: MilestoneNinePerturbationClass::EmployeeRecordFixturePolicyBasis,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_employee.clone(),
            hostile_lane: phase_four_employee.clone(),
            parity_lane: phase_four_employee.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "tenant-alpha-versus-tenant-beta-schema",
            perturbation_class: MilestoneNinePerturbationClass::TenantAlphaVersusTenantBetaSchema,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_four_employee.clone(),
            hostile_lane: phase_four_tenant_beta.clone(),
            parity_lane: phase_four_tenant_beta,
        },
        MilestoneNineCertificationRow {
            row_name: "masked-versus-unmasked-policy-parity",
            perturbation_class: MilestoneNinePerturbationClass::MaskedVersusUnmaskedPolicyParity,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_four_unmasked_policy,
            hostile_lane: phase_four_mask_parity.clone(),
            parity_lane: phase_four_mask_parity,
        },
        MilestoneNineCertificationRow {
            row_name: "delivery-width-class-honesty",
            perturbation_class: MilestoneNinePerturbationClass::DeliveryWidthClassHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_delivery_width.clone(),
            hostile_lane: phase_four_delivery_width.clone(),
            parity_lane: phase_four_delivery_width,
        },
        MilestoneNineCertificationRow {
            row_name: "live-policy-epoch-drift-readmission",
            perturbation_class: MilestoneNinePerturbationClass::LivePolicyEpochDriftReadmission,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_four_live_density.clone(),
            hostile_lane: phase_four_live_drift.clone(),
            parity_lane: phase_four_live_drift,
        },
        MilestoneNineCertificationRow {
            row_name: "live-policy-density-posture-honesty",
            perturbation_class: MilestoneNinePerturbationClass::LivePolicyDensityPostureHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_live_density.clone(),
            hostile_lane: phase_four_live_density.clone(),
            parity_lane: phase_four_live_density,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-scale-slope-honesty",
            perturbation_class: MilestoneNinePerturbationClass::PolicyScaleSlopeHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_scale.clone(),
            hostile_lane: phase_four_scale.clone(),
            parity_lane: phase_four_scale,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-direct-scope-template-saved-parity",
            perturbation_class:
                MilestoneNinePerturbationClass::PolicyDirectScopeTemplateSavedParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_composition.clone(),
            hostile_lane: phase_four_composition.clone(),
            parity_lane: phase_four_composition,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-view-shape-delivery-parity",
            perturbation_class: MilestoneNinePerturbationClass::PolicyViewShapeDeliveryParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_view_shape.clone(),
            hostile_lane: phase_four_view_shape.clone(),
            parity_lane: phase_four_view_shape,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-identity-aware-inspector-parity",
            perturbation_class: MilestoneNinePerturbationClass::PolicyIdentityAwareInspectorParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_four_identity_inspector.clone(),
            hostile_lane: phase_four_identity_inspector.clone(),
            parity_lane: phase_four_identity_inspector,
        },
    ]
}

fn bundle_digest_parts(matrix: &MilestoneNineCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    for row in &matrix.rows {
        parts.push(format!("row:{}", row.row_name));
        parts.push(format!(
            "control:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            row.control_lane.canonical_query_digest,
            row.control_lane.policy_digest,
            row.control_lane.result_digest,
            row.control_lane.tenant_truth_basis_digest,
            row.control_lane.execution_mode,
            row.control_lane.admission_disposition,
            row.control_lane.policy_cost_posture,
            row.control_lane.policy_work_budget_digest,
            row.control_lane.authorized_projection_digest,
            row.control_lane.relationship_proof_digest,
            row.control_lane.validation_report_digest,
            row.control_lane.policy_plan_digest,
            row.control_lane.policy_execution_seam_digest,
            row.control_lane.delivery_digest,
            row.control_lane.employee_fixture_digest,
            row.control_lane.policy_scale_counter_slope_digest,
            row.control_lane.live_drift_evidence_digest,
            row.control_lane.delivery_width_class_digest,
            row.control_lane.composition_policy_parity_digest,
            row.control_lane.view_shape_policy_parity_digest,
            row.control_lane.placeholder_denial_digest,
        ));
        parts.push(format!(
            "hostile:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            row.hostile_lane.canonical_query_digest,
            row.hostile_lane.policy_digest,
            row.hostile_lane.result_digest,
            row.hostile_lane.tenant_truth_basis_digest,
            row.hostile_lane.execution_mode,
            row.hostile_lane.admission_disposition,
            row.hostile_lane.policy_cost_posture,
            row.hostile_lane.policy_work_budget_digest,
            row.hostile_lane.authorized_projection_digest,
            row.hostile_lane.relationship_proof_digest,
            row.hostile_lane.validation_report_digest,
            row.hostile_lane.policy_plan_digest,
            row.hostile_lane.policy_execution_seam_digest,
            row.hostile_lane.delivery_digest,
            row.hostile_lane.employee_fixture_digest,
            row.hostile_lane.policy_scale_counter_slope_digest,
            row.hostile_lane.live_drift_evidence_digest,
            row.hostile_lane.delivery_width_class_digest,
            row.hostile_lane.composition_policy_parity_digest,
            row.hostile_lane.view_shape_policy_parity_digest,
            row.hostile_lane.placeholder_denial_digest,
        ));
    }
    for row in &matrix.rejection_rows {
        parts.push(format!(
            "rejection:{}:{}",
            row.row_name, row.hostile_lane.failure_digest
        ));
    }
    parts
}

fn coverage_digest_parts(matrix: &MilestoneNineCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    parts.extend(
        matrix
            .rows
            .iter()
            .map(|row| format!("row:{}", row.row_name)),
    );
    parts.extend(
        matrix
            .rejection_rows
            .iter()
            .map(|row| format!("rejection:{}", row.row_name)),
    );
    parts
}
