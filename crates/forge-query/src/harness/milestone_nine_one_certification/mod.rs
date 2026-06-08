mod tests;

use crate::harness::certification::{
    digest_parts, CanonicalCertificationRow, CertificationMatrix, HostileExpectation, ParityAnchor,
    RejectionCertificationRow,
};
use crate::live::LiveQueryFamily;
use crate::subscription::{
    admit_query_subscription, certify_query_subscription_activation,
    certify_query_subscription_scale_slope, declare_query_subscription,
    lower_query_subscription_to_bridge, prepare_subscription_activation,
    select_query_subscription_family, LiveQueryAdmissionArtifact, QuerySubscriptionAdmissionBudget,
    QuerySubscriptionBridgeLoweringBudget, QuerySubscriptionCertificationDenialKind,
    QuerySubscriptionConstructionSource, QuerySubscriptionDiagnosticStage,
    QuerySubscriptionRelationshipProofPosture, QuerySubscriptionScaleCounterSnapshot,
    QuerySubscriptionScaleFixtureSize, QuerySubscriptionSliceBudget, QuerySubscriptionWorkBudget,
};
use crate::view_shape_live::LiveViewShapeFamily;

pub const MILESTONE_NINE_ONE_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "detail-direct-scope-template-saved-facade-parity",
    "direct-scope-template-saved-subscription-parity",
    "facade-helper-subscription-parity",
    "collection-table-bridge-lowering-parity",
    "grouped-query-meaning-shares-collection-bridge-family",
    "inspector-query-meaning-shares-detail-bridge-family",
    "bounded-materialization-relation-scope-lowering",
    "activation-certification-source-binding",
    "basis-request-binds-policy-tenant-meaning",
    "relationship-proof-binds-subscription-meaning",
    "scale-slope-row-count-only-honesty",
];

pub const MILESTONE_NINE_ONE_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "view-family-mismatch-denies-before-declaration",
    "bridge-family-unsupported-denies-before-admission",
    "masked-detail-slice-denies-before-bridge-lowering",
    "masked-table-ordering-denies-before-bridge-lowering",
    "masked-grouped-membership-denies-before-bridge-lowering",
    "broken-relationship-proof-denies-before-bridge-lowering",
    "durable-reload-overclaim-denies-before-activation",
    "scale-report-source-mismatch-denies-certification",
    "scale-zero-row-baseline-denied",
];

pub const MILESTONE_NINE_ONE_REQUIRED_COMPILE_FAIL_TARGETS: &[&str] = &[
    "subscription_declaration_constructor_private.rs",
    "subscription_family_selection_constructor_private.rs",
    "subscription_bridge_lowering_plan_constructor_private.rs",
    "subscription_admission_artifact_constructor_private.rs",
    "subscription_activation_input_constructor_private.rs",
    "subscription_raw_live_descriptor_activation_forbidden.rs",
    "subscription_raw_bridge_declaration_activation_forbidden.rs",
    "subscription_raw_cdc_filter_declaration_forbidden.rs",
    "subscription_host_observer_callback_forbidden.rs",
    "subscription_bool_family_shortcut_forbidden.rs",
    "subscription_masked_slice_intent_constructor_forbidden.rs",
    "subscription_saved_exact_reuse_without_equivalence_forbidden.rs",
    "subscription_bridge_basis_request_without_query_basis_forbidden.rs",
    "subscription_policy_digest_patch_forbidden.rs",
    "subscription_tenant_digest_patch_forbidden.rs",
    "subscription_relationship_proof_digest_patch_forbidden.rs",
    "subscription_durable_reload_admission_forbidden.rs",
    "subscription_generic_kind_fallback_forbidden.rs",
    "subscription_diagnostic_evidence_constructor_private.rs",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MilestoneNineOnePerturbationClass {
    ConstructionSourceParity,
    RepresentativeConstructionSourceParity,
    FacadeHelperParity,
    CollectionBridgeLoweringParity,
    GroupedQueryMeaning,
    InspectorQueryMeaning,
    BoundedMaterializationLowering,
    ActivationCertificationSourceBinding,
    BasisRequestPolicyTenantBinding,
    RelationshipProofBinding,
    ScaleSlopeHonesty,
    ViewFamilyMismatch,
    BridgeFamilyUnsupported,
    MaskedDetailSlice,
    MaskedTableOrderingSlice,
    MaskedGroupedMembershipSlice,
    BrokenRelationshipProof,
    DurableReloadOverclaim,
    ScaleReportSourceMismatch,
    ScaleZeroRowBaseline,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneNineOneFailureClass {
    FamilySelectionDenied,
    DeclarationDenied,
    BridgeLoweringDenied,
    AdmissionDenied,
    CertificationDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNineOneCertificationBundle {
    pub query_digest: String,
    pub live_family_digest: String,
    pub subscription_family_digest: String,
    pub subscription_equivalence_digest: String,
    pub policy_digest: String,
    pub tenant_basis_digest: String,
    pub relationship_proof_digest: String,
    pub view_shape_digest: String,
    pub basis_digest: String,
    pub query_family: String,
    pub bridge_family: String,
    pub basis_request_digest: String,
    pub signal_strategy_digest: String,
    pub declaration_digest: String,
    pub bridge_declaration_digest: String,
    pub admission_digest: String,
    pub activation_digest: String,
    pub certification_bundle_digest: String,
    pub support_profile_digest: String,
    pub diagnostics_digest: String,
    pub scale_slope_digest: String,
    pub scale_activation_digest: String,
    pub scale_admission_digest: String,
    pub counter_snapshot_digest: String,
    pub fixture_digest: String,
    pub compile_fail_boundary_digest: String,
    pub support_matrix_digest: String,
}

impl MilestoneNineOneCertificationBundle {
    fn has_required_outputs(&self) -> bool {
        !self.query_digest.is_empty()
            && !self.live_family_digest.is_empty()
            && !self.subscription_family_digest.is_empty()
            && !self.subscription_equivalence_digest.is_empty()
            && !self.policy_digest.is_empty()
            && !self.tenant_basis_digest.is_empty()
            && !self.relationship_proof_digest.is_empty()
            && !self.view_shape_digest.is_empty()
            && !self.basis_digest.is_empty()
            && !self.query_family.is_empty()
            && !self.bridge_family.is_empty()
            && !self.basis_request_digest.is_empty()
            && !self.signal_strategy_digest.is_empty()
            && !self.declaration_digest.is_empty()
            && !self.bridge_declaration_digest.is_empty()
            && !self.admission_digest.is_empty()
            && !self.activation_digest.is_empty()
            && !self.certification_bundle_digest.is_empty()
            && !self.support_profile_digest.is_empty()
            && !self.diagnostics_digest.is_empty()
            && !self.scale_slope_digest.is_empty()
            && !self.scale_activation_digest.is_empty()
            && !self.scale_admission_digest.is_empty()
            && !self.counter_snapshot_digest.is_empty()
            && !self.fixture_digest.is_empty()
            && !self.compile_fail_boundary_digest.is_empty()
            && !self.support_matrix_digest.is_empty()
    }

    fn subscription_semantic_signature(&self) -> String {
        digest_parts(&[
            format!("query_family:{}", self.query_family),
            format!("bridge_family:{}", self.bridge_family),
            format!("subscription_family:{}", self.subscription_family_digest),
            format!("equivalence:{}", self.subscription_equivalence_digest),
            format!("policy:{}", self.policy_digest),
            format!("tenant:{}", self.tenant_basis_digest),
            format!("relationship_proof:{}", self.relationship_proof_digest),
            format!("view_shape:{}", self.view_shape_digest),
            format!("basis:{}", self.basis_request_digest),
            format!("signal:{}", self.signal_strategy_digest),
            format!("declaration:{}", self.declaration_digest),
            format!("bridge_declaration:{}", self.bridge_declaration_digest),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNineOneRejectionBundle {
    pub failure_class: MilestoneNineOneFailureClass,
    pub failure_kind: String,
    pub diagnostic_stage: String,
    pub diagnostic_digest: String,
    pub support_profile_digest: String,
    pub failure_digest: String,
    pub counter_snapshot_digest: String,
}

pub type MilestoneNineOneCertificationRow = CanonicalCertificationRow<
    MilestoneNineOnePerturbationClass,
    MilestoneNineOneCertificationBundle,
>;
pub type MilestoneNineOneRejectionRow = RejectionCertificationRow<
    MilestoneNineOnePerturbationClass,
    MilestoneNineOneCertificationBundle,
    MilestoneNineOneRejectionBundle,
>;
pub type MilestoneNineOneCertificationMatrix = CertificationMatrix<
    MilestoneNineOnePerturbationClass,
    MilestoneNineOneCertificationBundle,
    MilestoneNineOneRejectionBundle,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNineOneCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub matrix: MilestoneNineOneCertificationMatrix,
}

impl MilestoneNineOneCertificationMatrix {
    pub fn into_milestone_nine_one_artifact(self) -> MilestoneNineOneCertificationArtifact {
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));
        MilestoneNineOneCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            matrix: self,
        }
    }
}

pub struct MilestoneNineOneCertificationAdapter;

impl MilestoneNineOneCertificationAdapter {
    pub fn query_subscription_declaration_and_lowering_certification_artifact(
    ) -> MilestoneNineOneCertificationArtifact {
        Self::query_subscription_declaration_and_lowering_parity_test()
            .into_milestone_nine_one_artifact()
    }

    pub fn query_subscription_declaration_and_lowering_parity_test(
    ) -> MilestoneNineOneCertificationMatrix {
        MilestoneNineOneCertificationMatrix {
            suite_name: "Query Subscription Declaration And Lowering Parity Test",
            rows: canonical_rows(),
            rejection_rows: rejection_rows(),
        }
    }
}

fn canonical_rows() -> Vec<MilestoneNineOneCertificationRow> {
    vec![
        MilestoneNineOneCertificationRow {
            row_name: "detail-direct-scope-template-saved-facade-parity",
            perturbation_class: MilestoneNineOnePerturbationClass::ConstructionSourceParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::TemplateInstantiated,
            ),
            parity_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::SavedExactReuse,
            ),
        },
        MilestoneNineOneCertificationRow {
            row_name: "direct-scope-template-saved-subscription-parity",
            perturbation_class:
                MilestoneNineOnePerturbationClass::RepresentativeConstructionSourceParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::ScopeExpanded,
            ),
            parity_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::TemplateInstantiated,
            ),
        },
        MilestoneNineOneCertificationRow {
            row_name: "facade-helper-subscription-parity",
            perturbation_class: MilestoneNineOnePerturbationClass::FacadeHelperParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::SavedExactReuse,
            ),
            parity_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
        MilestoneNineOneCertificationRow {
            row_name: "collection-table-bridge-lowering-parity",
            perturbation_class: MilestoneNineOnePerturbationClass::CollectionBridgeLoweringParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::ScopeExpanded,
            ),
            parity_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
        MilestoneNineOneCertificationRow {
            row_name: "grouped-query-meaning-shares-collection-bridge-family",
            perturbation_class: MilestoneNineOnePerturbationClass::GroupedQueryMeaning,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
            hostile_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::KanbanGrouped),
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
            parity_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::KanbanGrouped),
                QuerySubscriptionConstructionSource::TemplateInstantiated,
            ),
        },
        MilestoneNineOneCertificationRow {
            row_name: "inspector-query-meaning-shares-detail-bridge-family",
            perturbation_class: MilestoneNineOnePerturbationClass::InspectorQueryMeaning,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: certified_lane(
                LiveQueryFamily::Detail,
                Some(LiveViewShapeFamily::Detail),
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
            hostile_lane: certified_lane(
                LiveQueryFamily::Detail,
                Some(LiveViewShapeFamily::InspectorDetailFocused),
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
            parity_lane: certified_lane(
                LiveQueryFamily::Detail,
                Some(LiveViewShapeFamily::InspectorDetailFocused),
                QuerySubscriptionConstructionSource::SavedExactReuse,
            ),
        },
        MilestoneNineOneCertificationRow {
            row_name: "bounded-materialization-relation-scope-lowering",
            perturbation_class: MilestoneNineOnePerturbationClass::BoundedMaterializationLowering,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: certified_lane(
                LiveQueryFamily::BoundedMaterialization,
                None,
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: certified_lane(
                LiveQueryFamily::BoundedMaterialization,
                None,
                QuerySubscriptionConstructionSource::ScopeExpanded,
            ),
            parity_lane: certified_lane(
                LiveQueryFamily::BoundedMaterialization,
                None,
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
        MilestoneNineOneCertificationRow {
            row_name: "activation-certification-source-binding",
            perturbation_class:
                MilestoneNineOnePerturbationClass::ActivationCertificationSourceBinding,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: certified_lane_with_basis(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::Direct,
                crate::subscription::QuerySubscriptionBasisPosture::BranchHead,
            ),
            parity_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
        MilestoneNineOneCertificationRow {
            row_name: "basis-request-binds-policy-tenant-meaning",
            perturbation_class: MilestoneNineOnePerturbationClass::BasisRequestPolicyTenantBinding,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: certified_lane_with_context(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::SavedExactReuse,
                "policy-alpha",
                "tenant-alpha",
                "relationship-proof",
            ),
            hostile_lane: certified_lane_with_context(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::SavedExactReuse,
                "policy-alpha",
                "tenant-beta",
                "relationship-proof",
            ),
            parity_lane: certified_lane_with_context(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::FacadeLive,
                "policy-alpha",
                "tenant-beta",
                "relationship-proof",
            ),
        },
        MilestoneNineOneCertificationRow {
            row_name: "relationship-proof-binds-subscription-meaning",
            perturbation_class: MilestoneNineOnePerturbationClass::RelationshipProofBinding,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: certified_lane_with_context(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::SavedExactReuse,
                "policy-alpha",
                "tenant-alpha",
                "relationship-proof-alpha",
            ),
            hostile_lane: certified_lane_with_context(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::SavedExactReuse,
                "policy-alpha",
                "tenant-alpha",
                "relationship-proof-beta",
            ),
            parity_lane: certified_lane_with_context(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::FacadeLive,
                "policy-alpha",
                "tenant-alpha",
                "relationship-proof-beta",
            ),
        },
        MilestoneNineOneCertificationRow {
            row_name: "scale-slope-row-count-only-honesty",
            perturbation_class: MilestoneNineOnePerturbationClass::ScaleSlopeHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: certified_lane_with_scale(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::Direct,
                [2, 20, 200],
            ),
            parity_lane: certified_lane_with_scale(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::FacadeLive,
                [3, 30, 300],
            ),
        },
    ]
}

fn rejection_rows() -> Vec<MilestoneNineOneRejectionRow> {
    vec![
        MilestoneNineOneRejectionRow {
            row_name: "view-family-mismatch-denies-before-declaration",
            perturbation_class: MilestoneNineOnePerturbationClass::ViewFamilyMismatch,
            control_lane: certified_lane(
                LiveQueryFamily::Detail,
                Some(LiveViewShapeFamily::Detail),
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: view_family_mismatch_rejection(),
            parity_lane: certified_lane(
                LiveQueryFamily::Detail,
                Some(LiveViewShapeFamily::Detail),
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
        MilestoneNineOneRejectionRow {
            row_name: "bridge-family-unsupported-denies-before-admission",
            perturbation_class: MilestoneNineOnePerturbationClass::BridgeFamilyUnsupported,
            control_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: bridge_family_rejection(),
            parity_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
        MilestoneNineOneRejectionRow {
            row_name: "masked-detail-slice-denies-before-bridge-lowering",
            perturbation_class: MilestoneNineOnePerturbationClass::MaskedDetailSlice,
            control_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: masked_slice_rejection(LiveQueryFamily::Detail, None),
            parity_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
        MilestoneNineOneRejectionRow {
            row_name: "masked-table-ordering-denies-before-bridge-lowering",
            perturbation_class: MilestoneNineOnePerturbationClass::MaskedTableOrderingSlice,
            control_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: masked_slice_rejection(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
            ),
            parity_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
        MilestoneNineOneRejectionRow {
            row_name: "masked-grouped-membership-denies-before-bridge-lowering",
            perturbation_class: MilestoneNineOnePerturbationClass::MaskedGroupedMembershipSlice,
            control_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::KanbanGrouped),
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: masked_slice_rejection(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::KanbanGrouped),
            ),
            parity_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::KanbanGrouped),
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
        MilestoneNineOneRejectionRow {
            row_name: "broken-relationship-proof-denies-before-bridge-lowering",
            perturbation_class: MilestoneNineOnePerturbationClass::BrokenRelationshipProof,
            control_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: broken_relationship_proof_rejection(),
            parity_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
        MilestoneNineOneRejectionRow {
            row_name: "durable-reload-overclaim-denies-before-activation",
            perturbation_class: MilestoneNineOnePerturbationClass::DurableReloadOverclaim,
            control_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: durable_reload_rejection(),
            parity_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
        MilestoneNineOneRejectionRow {
            row_name: "scale-report-source-mismatch-denies-certification",
            perturbation_class: MilestoneNineOnePerturbationClass::ScaleReportSourceMismatch,
            control_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: scale_source_mismatch_rejection(),
            parity_lane: certified_lane(
                LiveQueryFamily::Detail,
                None,
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
        MilestoneNineOneRejectionRow {
            row_name: "scale-zero-row-baseline-denied",
            perturbation_class: MilestoneNineOnePerturbationClass::ScaleZeroRowBaseline,
            control_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::Direct,
            ),
            hostile_lane: scale_zero_row_rejection(),
            parity_lane: certified_lane(
                LiveQueryFamily::OrderedCollection,
                Some(LiveViewShapeFamily::Table),
                QuerySubscriptionConstructionSource::FacadeLive,
            ),
        },
    ]
}

fn certified_lane(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    construction_source: QuerySubscriptionConstructionSource,
) -> MilestoneNineOneCertificationBundle {
    certified_lane_with_scale(
        live_family,
        view_family,
        construction_source,
        [10, 100, 1000],
    )
}

fn certified_lane_with_basis(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    construction_source: QuerySubscriptionConstructionSource,
    basis_posture: crate::subscription::QuerySubscriptionBasisPosture,
) -> MilestoneNineOneCertificationBundle {
    certified_lane_from_live(
        LiveQueryAdmissionArtifact::for_test_with_basis(
            live_family,
            view_family,
            construction_source,
            basis_posture,
        ),
        [10, 100, 1000],
    )
}

fn certified_lane_with_scale(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    construction_source: QuerySubscriptionConstructionSource,
    row_counts: [u64; 3],
) -> MilestoneNineOneCertificationBundle {
    certified_lane_from_live(
        LiveQueryAdmissionArtifact::for_test(live_family, view_family, construction_source),
        row_counts,
    )
}

fn certified_lane_with_context(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    construction_source: QuerySubscriptionConstructionSource,
    policy_digest: &str,
    tenant_digest: &str,
    relationship_proof_digest: &str,
) -> MilestoneNineOneCertificationBundle {
    certified_lane_from_live(
        LiveQueryAdmissionArtifact::for_test_with_context(
            live_family,
            view_family,
            construction_source,
            crate::subscription::QuerySubscriptionBasisPosture::CurrentHead,
            crate::subscription::QuerySubscriptionFutureSelection::ordinary(),
            Some(policy_digest.to_string()),
            Some(tenant_digest.to_string()),
            Some(relationship_proof_digest.to_string()),
            QuerySubscriptionRelationshipProofPosture::Admitted,
        ),
        [10, 100, 1000],
    )
}

fn certified_lane_from_live(
    live: LiveQueryAdmissionArtifact,
    row_counts: [u64; 3],
) -> MilestoneNineOneCertificationBundle {
    let query_digest = live.query_digest().to_string();
    let live_family_digest =
        digest_parts(&[format!("live_family:{}", live.live_family().as_str())]);
    let policy_digest = live.policy_digest().unwrap_or("none").to_string();
    let tenant_basis_digest = live.tenant_digest().unwrap_or("none").to_string();
    let relationship_proof_digest = live
        .relationship_proof_digest()
        .unwrap_or("none")
        .to_string();
    let view_shape_digest = digest_parts(&[format!(
        "view_shape:{}",
        live.view_family()
            .map(|family| family.as_str())
            .unwrap_or("none")
    )]);
    let basis_digest = digest_parts(&[format!("basis:{}", live.basis_posture().as_str())]);
    let fixture_digest = digest_parts(&[
        format!("family:{}", live.live_family().as_str()),
        format!(
            "view:{}",
            live.view_family()
                .map(|family| family.as_str())
                .unwrap_or("none")
        ),
        format!("source:{}", live.construction_source().as_str()),
        format!("rows:{row_counts:?}"),
    ]);
    let selection = select_query_subscription_family(live, work_budget()).unwrap();
    let subscription_family_digest = digest_parts(&[format!(
        "subscription_family:{}",
        selection.family().as_str()
    )]);
    let subscription_equivalence_digest =
        selection.equivalence_basis().digest().as_str().to_string();
    let declaration = declare_query_subscription(selection, slice_budget()).unwrap();
    let query_family = declaration.family().as_str().to_string();
    let declaration_digest = declaration.declaration_digest().as_str().to_string();
    let lowering = lower_query_subscription_to_bridge(declaration, lowering_budget()).unwrap();
    let bridge_family = lowering.bridge_family().as_str().to_string();
    let bridge_declaration_digest = lowering.bridge_declaration_digest().to_string();
    let basis_request_digest = lowering.basis_request().digest().to_string();
    let signal_strategy_digest = lowering.signal_strategy_request().digest().to_string();
    let admission = admit_query_subscription(lowering, admission_budget()).unwrap();
    let support_profile_digest = admission.support_profile().digest().to_string();
    let diagnostics_digest = admission.diagnostics().digest().to_string();
    let support_matrix_digest = digest_parts(&[
        format!("support:{}", support_profile_digest),
        format!("diagnostics:{}", diagnostics_digest),
    ]);
    let activation = prepare_subscription_activation(admission.clone());
    let scale_report = certify_query_subscription_scale_slope(
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Small,
            row_counts[0],
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Medium,
            row_counts[1],
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Large,
            row_counts[2],
            &activation,
        ),
    )
    .unwrap();
    let certification =
        certify_query_subscription_activation(admission, activation, scale_report).unwrap();

    MilestoneNineOneCertificationBundle {
        query_digest,
        live_family_digest,
        subscription_family_digest,
        subscription_equivalence_digest,
        policy_digest,
        tenant_basis_digest,
        relationship_proof_digest,
        view_shape_digest,
        basis_digest,
        query_family,
        bridge_family,
        basis_request_digest,
        signal_strategy_digest,
        declaration_digest,
        bridge_declaration_digest,
        admission_digest: certification.admission_digest().to_string(),
        activation_digest: certification.activation_digest().to_string(),
        certification_bundle_digest: certification.certification_bundle_digest().to_string(),
        support_profile_digest,
        diagnostics_digest,
        scale_slope_digest: certification.scale_slope_digest().to_string(),
        scale_activation_digest: certification.scale_activation_digest().to_string(),
        scale_admission_digest: certification.scale_admission_digest().to_string(),
        counter_snapshot_digest: digest_parts(&[
            format!(
                "admission_counters:{}",
                certification.admission_counter_digest()
            ),
            format!(
                "activation_counters:{}",
                certification.activation_counter_digest()
            ),
        ]),
        fixture_digest,
        compile_fail_boundary_digest: compile_fail_boundary_digest(),
        support_matrix_digest,
    }
}

fn view_family_mismatch_rejection() -> MilestoneNineOneRejectionBundle {
    let live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::Detail,
        Some(LiveViewShapeFamily::Table),
        QuerySubscriptionConstructionSource::Direct,
    );
    let error = select_query_subscription_family(live, work_budget()).unwrap_err();
    rejection(
        MilestoneNineOneFailureClass::FamilySelectionDenied,
        error.failure_class().as_str(),
        error.diagnostic().stage().as_str(),
        error.diagnostic().digest(),
        "",
        &[
            format!("message:{}", error.message()),
            format!("diagnostic:{}", error.diagnostic().digest()),
            format!("counters:{}", error.counters().digest()),
        ],
        error.counters().digest(),
    )
}

fn bridge_family_rejection() -> MilestoneNineOneRejectionBundle {
    let live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
        QuerySubscriptionConstructionSource::Direct,
    );
    let selection = select_query_subscription_family(live, work_budget()).unwrap();
    let declaration = declare_query_subscription(selection, slice_budget()).unwrap();
    let error = lower_query_subscription_to_bridge(
        declaration,
        lowering_budget().without_bridge_family_support(),
    )
    .unwrap_err();
    rejection(
        MilestoneNineOneFailureClass::BridgeLoweringDenied,
        error.denial_kind().as_str(),
        error.diagnostic().stage().as_str(),
        error.diagnostic().digest(),
        "",
        &[
            format!("message:{}", error.message()),
            format!("diagnostic:{}", error.diagnostic().digest()),
            format!("counters:{}", error.counters().digest()),
        ],
        error.counters().digest(),
    )
}

fn masked_slice_rejection(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
) -> MilestoneNineOneRejectionBundle {
    let live = LiveQueryAdmissionArtifact::for_test(
        live_family,
        view_family,
        QuerySubscriptionConstructionSource::Direct,
    );
    let selection = select_query_subscription_family(live, work_budget()).unwrap();
    let error = declare_query_subscription(
        selection,
        slice_budget().with_masked_slice_request_detected(),
    )
    .unwrap_err();
    rejection(
        MilestoneNineOneFailureClass::DeclarationDenied,
        error.denial_kind().as_str(),
        error.diagnostic().stage().as_str(),
        error.diagnostic().digest(),
        "",
        &[
            format!("message:{}", error.message()),
            format!("diagnostic:{}", error.diagnostic().digest()),
            format!("counters:{}", error.counters().digest()),
        ],
        error.counters().digest(),
    )
}

fn broken_relationship_proof_rejection() -> MilestoneNineOneRejectionBundle {
    let live = LiveQueryAdmissionArtifact::for_test_with_relationship_proof_posture(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::SavedExactReuse,
        QuerySubscriptionRelationshipProofPosture::Drifted,
    );
    let error = select_query_subscription_family(live, work_budget()).unwrap_err();
    rejection(
        MilestoneNineOneFailureClass::FamilySelectionDenied,
        error.failure_class().as_str(),
        error.diagnostic().stage().as_str(),
        error.diagnostic().digest(),
        "",
        &[
            format!("message:{}", error.message()),
            format!("diagnostic:{}", error.diagnostic().digest()),
            format!("counters:{}", error.counters().digest()),
        ],
        error.counters().digest(),
    )
}

fn durable_reload_rejection() -> MilestoneNineOneRejectionBundle {
    let live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::Direct,
    );
    let selection = select_query_subscription_family(live, work_budget()).unwrap();
    let declaration = declare_query_subscription(selection, slice_budget()).unwrap();
    let lowering = lower_query_subscription_to_bridge(declaration, lowering_budget()).unwrap();
    let error =
        admit_query_subscription(lowering, admission_budget().with_durable_reload_request())
            .unwrap_err();
    rejection(
        MilestoneNineOneFailureClass::AdmissionDenied,
        error.denial_kind().as_str(),
        error.pipeline_diagnostic().stage().as_str(),
        error.pipeline_diagnostic().digest(),
        error.support_profile().digest(),
        &[
            format!("message:{}", error.message()),
            format!("diagnostics:{}", error.diagnostics().digest()),
            format!(
                "pipeline_diagnostic:{}",
                error.pipeline_diagnostic().digest()
            ),
            format!("support:{}", error.support_profile().digest()),
            format!("counters:{}", error.counters().digest()),
        ],
        error.counters().digest(),
    )
}

fn scale_source_mismatch_rejection() -> MilestoneNineOneRejectionBundle {
    let source = admitted_activation(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::Direct,
    );
    let foreign = admitted_activation(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
        QuerySubscriptionConstructionSource::Direct,
    );
    let foreign_scale = certify_query_subscription_scale_slope(
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Small,
            10,
            &foreign.1,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Medium,
            100,
            &foreign.1,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Large,
            1000,
            &foreign.1,
        ),
    )
    .unwrap();
    let error =
        certify_query_subscription_activation(source.0, source.1, foreign_scale).unwrap_err();
    debug_assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionCertificationDenialKind::ScaleSlopeSourceMismatch
    );
    rejection(
        MilestoneNineOneFailureClass::CertificationDenied,
        error.denial_kind().as_str(),
        QuerySubscriptionDiagnosticStage::Certification.as_str(),
        error.failure_digest(),
        "",
        &[
            format!("message:{}", error.message()),
            format!("failure:{}", error.failure_digest()),
        ],
        error.failure_digest().to_string(),
    )
}

fn scale_zero_row_rejection() -> MilestoneNineOneRejectionBundle {
    let (_, activation) = admitted_activation(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
        QuerySubscriptionConstructionSource::Direct,
    );
    let error = certify_query_subscription_scale_slope(
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Small,
            0,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Medium,
            10,
            &activation,
        ),
        QuerySubscriptionScaleCounterSnapshot::from_activation(
            QuerySubscriptionScaleFixtureSize::Large,
            100,
            &activation,
        ),
    )
    .unwrap_err();
    rejection(
        MilestoneNineOneFailureClass::CertificationDenied,
        error.denial_kind().as_str(),
        QuerySubscriptionDiagnosticStage::Certification.as_str(),
        error.failure_digest(),
        "",
        &[
            format!("message:{}", error.message()),
            format!("failure:{}", error.failure_digest()),
        ],
        error.failure_digest().to_string(),
    )
}

fn admitted_activation(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    construction_source: QuerySubscriptionConstructionSource,
) -> (
    crate::subscription::QuerySubscriptionAdmissionArtifact,
    crate::subscription::SubscriptionActivationInput,
) {
    let live = LiveQueryAdmissionArtifact::for_test(live_family, view_family, construction_source);
    let selection = select_query_subscription_family(live, work_budget()).unwrap();
    let declaration = declare_query_subscription(selection, slice_budget()).unwrap();
    let lowering = lower_query_subscription_to_bridge(declaration, lowering_budget()).unwrap();
    let admission = admit_query_subscription(lowering, admission_budget()).unwrap();
    let activation = prepare_subscription_activation(admission.clone());
    (admission, activation)
}

fn rejection(
    failure_class: MilestoneNineOneFailureClass,
    failure_kind: &str,
    diagnostic_stage: &str,
    diagnostic_digest: &str,
    support_profile_digest: &str,
    evidence_parts: &[String],
    counter_snapshot_digest: String,
) -> MilestoneNineOneRejectionBundle {
    let mut parts = vec![
        "milestone_nine_one_rejection_v1".to_string(),
        format!("failure_class:{failure_class:?}"),
        format!("failure_kind:{failure_kind}"),
        format!("diagnostic_stage:{diagnostic_stage}"),
        format!("diagnostic:{diagnostic_digest}"),
        format!("support:{support_profile_digest}"),
    ];
    parts.extend(evidence_parts.iter().cloned());
    MilestoneNineOneRejectionBundle {
        failure_class,
        failure_kind: failure_kind.to_string(),
        diagnostic_stage: diagnostic_stage.to_string(),
        diagnostic_digest: diagnostic_digest.to_string(),
        support_profile_digest: support_profile_digest.to_string(),
        failure_digest: digest_parts(&parts),
        counter_snapshot_digest,
    }
}

fn bundle_digest_parts(matrix: &MilestoneNineOneCertificationMatrix) -> Vec<String> {
    matrix
        .rows
        .iter()
        .flat_map(|row| {
            [
                format!(
                    "{}:control:{}",
                    row.row_name, row.control_lane.certification_bundle_digest
                ),
                format!(
                    "{}:hostile:{}",
                    row.row_name, row.hostile_lane.certification_bundle_digest
                ),
                format!(
                    "{}:parity:{}",
                    row.row_name, row.parity_lane.certification_bundle_digest
                ),
            ]
        })
        .chain(matrix.rejection_rows.iter().flat_map(|row| {
            [
                format!(
                    "{}:control:{}",
                    row.row_name, row.control_lane.certification_bundle_digest
                ),
                format!(
                    "{}:hostile:{}",
                    row.row_name, row.hostile_lane.failure_digest
                ),
                format!(
                    "{}:parity:{}",
                    row.row_name, row.parity_lane.certification_bundle_digest
                ),
            ]
        }))
        .collect()
}

fn coverage_digest_parts(matrix: &MilestoneNineOneCertificationMatrix) -> Vec<String> {
    matrix
        .rows
        .iter()
        .map(|row| {
            format!(
                "canonical:{}:{:?}:{:?}:{:?}",
                row.row_name, row.perturbation_class, row.hostile_expectation, row.parity_anchor
            )
        })
        .chain(
            matrix
                .rejection_rows
                .iter()
                .map(|row| format!("rejection:{}:{:?}", row.row_name, row.perturbation_class)),
        )
        .collect()
}

fn compile_fail_boundary_digest() -> String {
    let mut parts = MILESTONE_NINE_ONE_REQUIRED_COMPILE_FAIL_TARGETS
        .iter()
        .flat_map(|target| {
            [
                format!("target:{target}"),
                format!(
                    "stderr:{}",
                    target.trim_end_matches(".rs").to_string() + ".stderr"
                ),
            ]
        })
        .collect::<Vec<_>>();
    parts.sort();
    digest_parts(&parts)
}

fn work_budget() -> QuerySubscriptionWorkBudget {
    QuerySubscriptionWorkBudget::scratch_buffer_only(8, 8, 8, 64, 1)
}

fn slice_budget() -> QuerySubscriptionSliceBudget {
    QuerySubscriptionSliceBudget::scratch_buffer_only(8, 8, 8, 8, 8, 8, 8, 8)
}

fn lowering_budget() -> QuerySubscriptionBridgeLoweringBudget {
    QuerySubscriptionBridgeLoweringBudget::admitted(1, 8, 8, 1, 1)
}

fn admission_budget() -> QuerySubscriptionAdmissionBudget {
    QuerySubscriptionAdmissionBudget::admitted(1, 1, 1, 1, 1)
}
