use crate::spatial_compiled_product_family::SpatialCompiledProductLoweredIdentity;
use crate::workload_platform::evidence_ledger::WorkloadEvidenceRow;
use crate::workload_platform::evidence_lookup_query_surface_contract::{
    EvidenceLookupProductQuerySurfaceContractRow, EvidenceLookupQuerySurfaceContract,
};
use crate::workload_platform::selected_equivalence_family::{
    SelectedSpatialEquivalenceFamily, SpatialCompatibilityPosture,
    SpatialFreshnessRequirementPosture, SpatialOrderingNoisePosture,
    SpatialRenderedOutputComparisonPosture, SpatialSelectedEquivalenceFamilyIdentity,
};

use super::counters::EvidenceLookupIndexProductCounters;
use super::disposal_posture::EvidenceLookupIndexDisposalPosture;
use super::identity::index_product_digest;
use super::lifecycle_posture::EvidenceLookupIndexLifecyclePosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupIndexProduct {
    index_product_digest: String,
    compiled_product_identity_digest: String,
    equivalence_policy_identity_digest: String,
    selected_equivalence_family_identity: SpatialSelectedEquivalenceFamilyIdentity,
    selected_equivalence_basis_identity_digest: String,
    selected_compatibility_basis_identity_digest: String,
    selected_reuse_basis_identity_digest: String,
    selected_compatibility_posture: SpatialCompatibilityPosture,
    selected_freshness_requirement_posture: SpatialFreshnessRequirementPosture,
    selected_ordering_noise_posture: SpatialOrderingNoisePosture,
    selected_rendered_output_comparison_posture: SpatialRenderedOutputComparisonPosture,
    reuse_decision_identity_digest: Option<String>,
    selected_plan_digest: String,
    spatial_touch_digest: String,
    stage_receipt_digest: String,
    evidence_ledger_basis_digest: String,
    topology_support_digest: String,
    query_support_digest: String,
    query_surface_contract_rows: Vec<EvidenceLookupProductQuerySurfaceContractRow>,
    lifecycle_posture: EvidenceLookupIndexLifecyclePosture,
    disposal_posture: EvidenceLookupIndexDisposalPosture,
    counters: EvidenceLookupIndexProductCounters,
    rows: Vec<WorkloadEvidenceRow>,
}

impl EvidenceLookupIndexProduct {
    pub(crate) fn new(
        lowered_identity: &SpatialCompiledProductLoweredIdentity,
        selected_equivalence_family: &SelectedSpatialEquivalenceFamily,
        selected_plan_digest: String,
        spatial_touch_digest: String,
        stage_receipt_digest: String,
        evidence_ledger_basis_digest: String,
        topology_support_digest: String,
        query_support_digest: String,
        reuse_decision_identity_digest: Option<String>,
        query_surface_contract_rows: Vec<EvidenceLookupProductQuerySurfaceContractRow>,
        lifecycle_posture: EvidenceLookupIndexLifecyclePosture,
        disposal_posture: EvidenceLookupIndexDisposalPosture,
        counters: EvidenceLookupIndexProductCounters,
        rows: Vec<WorkloadEvidenceRow>,
    ) -> Self {
        let index_product_digest = index_product_digest(
            lowered_identity.compiled_product_identity(),
            selected_equivalence_family,
            lifecycle_posture,
            disposal_posture,
            &counters,
        );
        Self {
            index_product_digest,
            compiled_product_identity_digest: lowered_identity
                .compiled_product_identity()
                .identity_digest()
                .to_string(),
            equivalence_policy_identity_digest: lowered_identity
                .equivalence_policy_identity()
                .identity_digest()
                .to_string(),
            selected_equivalence_family_identity: selected_equivalence_family.family_identity(),
            selected_equivalence_basis_identity_digest: selected_equivalence_family
                .equivalence_basis_identity()
                .identity_digest()
                .to_string(),
            selected_compatibility_basis_identity_digest: selected_equivalence_family
                .compatibility_basis_identity()
                .identity_digest()
                .to_string(),
            selected_reuse_basis_identity_digest: selected_equivalence_family
                .reuse_basis_identity()
                .identity_digest()
                .to_string(),
            selected_compatibility_posture: selected_equivalence_family.compatibility_posture(),
            selected_freshness_requirement_posture: selected_equivalence_family
                .freshness_requirement_posture(),
            selected_ordering_noise_posture: selected_equivalence_family.ordering_noise_posture(),
            selected_rendered_output_comparison_posture: selected_equivalence_family
                .rendered_output_comparison_posture(),
            reuse_decision_identity_digest,
            selected_plan_digest,
            spatial_touch_digest,
            stage_receipt_digest,
            evidence_ledger_basis_digest,
            topology_support_digest,
            query_support_digest,
            query_surface_contract_rows,
            lifecycle_posture,
            disposal_posture,
            counters,
            rows,
        }
    }

    pub fn index_product_digest(&self) -> &str {
        &self.index_product_digest
    }

    pub fn compiled_product_identity_digest(&self) -> &str {
        &self.compiled_product_identity_digest
    }

    pub fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
    }

    pub const fn selected_equivalence_family_identity(
        &self,
    ) -> SpatialSelectedEquivalenceFamilyIdentity {
        self.selected_equivalence_family_identity
    }

    pub fn selected_equivalence_basis_identity_digest(&self) -> &str {
        &self.selected_equivalence_basis_identity_digest
    }

    pub fn selected_compatibility_basis_identity_digest(&self) -> &str {
        &self.selected_compatibility_basis_identity_digest
    }

    pub fn selected_reuse_basis_identity_digest(&self) -> &str {
        &self.selected_reuse_basis_identity_digest
    }

    pub const fn selected_compatibility_posture(&self) -> SpatialCompatibilityPosture {
        self.selected_compatibility_posture
    }

    pub const fn selected_freshness_requirement_posture(
        &self,
    ) -> SpatialFreshnessRequirementPosture {
        self.selected_freshness_requirement_posture
    }

    pub const fn selected_ordering_noise_posture(&self) -> SpatialOrderingNoisePosture {
        self.selected_ordering_noise_posture
    }

    pub const fn selected_rendered_output_comparison_posture(
        &self,
    ) -> SpatialRenderedOutputComparisonPosture {
        self.selected_rendered_output_comparison_posture
    }

    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.reuse_decision_identity_digest.as_deref()
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }

    pub fn stage_receipt_digest(&self) -> &str {
        &self.stage_receipt_digest
    }

    pub fn evidence_ledger_basis_digest(&self) -> &str {
        &self.evidence_ledger_basis_digest
    }

    pub fn topology_support_digest(&self) -> &str {
        &self.topology_support_digest
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn query_surface_contract_rows(&self) -> &[EvidenceLookupProductQuerySurfaceContractRow] {
        &self.query_surface_contract_rows
    }

    pub fn query_surface_contract_for_family(
        &self,
        family_identity: &str,
    ) -> Option<&EvidenceLookupQuerySurfaceContract> {
        self.query_surface_contract_rows
            .iter()
            .find(|row| row.family_identity() == family_identity)
            .map(EvidenceLookupProductQuerySurfaceContractRow::contract)
    }

    pub const fn lifecycle_posture(&self) -> EvidenceLookupIndexLifecyclePosture {
        self.lifecycle_posture
    }

    pub const fn disposal_posture(&self) -> EvidenceLookupIndexDisposalPosture {
        self.disposal_posture
    }

    pub const fn counters(&self) -> &EvidenceLookupIndexProductCounters {
        &self.counters
    }

    pub const fn claims_lookup_execution(&self) -> bool {
        false
    }

    pub const fn claims_persistent_capability(&self) -> bool {
        self.lifecycle_posture.claims_persistent_capability()
    }

    pub const fn claims_query_descriptor_authority(&self) -> bool {
        false
    }

    pub(crate) fn rows(&self) -> &[WorkloadEvidenceRow] {
        &self.rows
    }

    #[cfg(any(test, feature = "test-support-lowering"))]
    pub fn with_test_selected_reuse_basis_identity_digest(
        mut self,
        selected_reuse_basis_identity_digest: impl Into<String>,
    ) -> Self {
        self.selected_reuse_basis_identity_digest = selected_reuse_basis_identity_digest.into();
        self
    }

    #[cfg(any(test, feature = "test-support-lowering"))]
    pub fn with_test_selected_plan_digest(
        mut self,
        selected_plan_digest: impl Into<String>,
    ) -> Self {
        self.selected_plan_digest = selected_plan_digest.into();
        self
    }

    #[cfg(any(test, feature = "test-support-lowering"))]
    pub fn with_test_selected_equivalence_family_identity(
        mut self,
        selected_equivalence_family_identity: impl AsRef<str>,
    ) -> Self {
        self.selected_equivalence_family_identity =
            match selected_equivalence_family_identity.as_ref() {
                "spatial.selected-equivalence.evidence-lookup-semantic-parity" => {
                    SpatialSelectedEquivalenceFamilyIdentity::EvidenceLookupSemanticParity
                }
                "spatial.selected-equivalence.retained-cancellation-semantic-parity" => {
                    SpatialSelectedEquivalenceFamilyIdentity::RetainedCancellationSemanticParity
                }
                "spatial.selected-equivalence.retained-replay-semantic-parity" => {
                    SpatialSelectedEquivalenceFamilyIdentity::RetainedReplaySemanticParity
                }
                other => panic!("unknown test selected equivalence family identity {other}"),
            };
        self
    }
}
