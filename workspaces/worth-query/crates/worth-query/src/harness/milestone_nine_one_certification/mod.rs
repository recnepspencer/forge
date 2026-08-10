mod bundle_projection;
mod canonical_rows;
mod lane_builders;
mod rejection_evidence;
mod rejection_rows;
#[cfg(test)]
mod tests;

use crate::harness::certification::{
    digest_parts, CanonicalCertificationRow, CertificationMatrix, RejectionCertificationRow,
};

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
        let certification_bundle_digest =
            digest_parts(&bundle_projection::bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&bundle_projection::coverage_digest_parts(&self));
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
            rows: canonical_rows::canonical_rows(),
            rejection_rows: rejection_rows::rejection_rows(),
        }
    }
}
