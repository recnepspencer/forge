use schema::facade::platform::authority::compiled_product_semantic_graph::{
    CompiledProductAuthorityTruthIdentity, CompiledProductEquivalencePolicyIdentity,
    CompiledProductIdentity,
};
use serde::{Deserialize, Serialize};

use crate::compiled_product_family::{DeterministicDigest, TopologyCompiledProductFamilyIdentity};
use crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport;
use crate::selected_equivalence_family::{
    TopologyOrderingNoisePosture, TopologyRenderedOutputComparisonPosture,
    TopologySelectedEquivalenceComparable, TopologySelectedEquivalenceComparatorContract,
    TopologySelectedEquivalenceComparisonReport, TopologySelectedEquivalenceFamilyIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TopologyDerivedReuseExecutionInput {
    authority_snapshot_id: u64,
    authority_branch_id: String,
    authoritative_mutation_origin: schema::facade::platform::authority::MutationOrigin,
    truth_basis_digest_hex: String,
    touched_aspect_count: usize,
    triggered_invalidation_targets:
        Vec<schema::facade::platform::authority::DerivedInvalidationTarget>,
    topology_compiled_product_family_identity: Option<TopologyCompiledProductFamilyIdentity>,
    topology_compiled_product_family_digest: Option<String>,
    compiled_product_identity: Option<CompiledProductIdentity>,
    equivalence_policy_identity: Option<CompiledProductEquivalencePolicyIdentity>,
    authority_truth_identity: Option<CompiledProductAuthorityTruthIdentity>,
    compiled_product_identity_digest: Option<String>,
    equivalence_policy_identity_digest: Option<String>,
    selected_equivalence_family_identity: Option<TopologySelectedEquivalenceFamilyIdentity>,
    selected_equivalence_basis_identity_digest: Option<String>,
    selected_compatibility_basis_identity_digest: Option<String>,
    selected_reuse_basis_identity_digest: Option<String>,
    selected_comparator_contract: Option<TopologySelectedEquivalenceComparatorContract>,
    selected_ordering_noise_posture: Option<TopologyOrderingNoisePosture>,
    selected_rendered_output_comparison_posture: Option<TopologyRenderedOutputComparisonPosture>,
    materialized_topology_digest: DeterministicDigest,
    interpreted_topology_digest: DeterministicDigest,
    derived_validation_digest: DeterministicDigest,
    authority_truth_identity_digest: Option<String>,
}

impl TopologyDerivedReuseExecutionInput {
    pub(crate) fn lower(report: &DerivedEquivalenceContractReport) -> Self {
        Self {
            authority_snapshot_id: report.authority_snapshot_id,
            authority_branch_id: report.authority_branch_id.clone(),
            authoritative_mutation_origin: report.authoritative_mutation_origin,
            truth_basis_digest_hex: report.truth_basis_digest_hex.clone(),
            touched_aspect_count: report.touched_aspect_count,
            triggered_invalidation_targets: report.triggered_invalidation_targets.clone(),
            topology_compiled_product_family_identity: report
                .topology_compiled_product_family_identity(),
            topology_compiled_product_family_digest: report
                .topology_compiled_product_family_digest()
                .map(str::to_string),
            compiled_product_identity: report.compiled_product_identity_ref().cloned(),
            equivalence_policy_identity: report.equivalence_policy_identity_ref().cloned(),
            authority_truth_identity: report.authority_truth_identity_ref().cloned(),
            compiled_product_identity_digest: report
                .compiled_product_identity_digest()
                .map(str::to_string),
            equivalence_policy_identity_digest: report
                .equivalence_policy_identity_digest()
                .map(str::to_string),
            selected_equivalence_family_identity: report.selected_equivalence_family_identity(),
            selected_equivalence_basis_identity_digest: report
                .selected_equivalence_basis_identity_digest()
                .map(str::to_string),
            selected_compatibility_basis_identity_digest: report
                .selected_compatibility_basis_identity_digest()
                .map(str::to_string),
            selected_reuse_basis_identity_digest: report
                .selected_reuse_basis_identity_digest()
                .map(str::to_string),
            selected_comparator_contract: report.selected_comparator_contract(),
            selected_ordering_noise_posture: report.selected_ordering_noise_posture(),
            selected_rendered_output_comparison_posture: report
                .selected_rendered_output_comparison_posture(),
            materialized_topology_digest: report.materialized_topology_digest.clone(),
            interpreted_topology_digest: report.interpreted_topology_digest.clone(),
            derived_validation_digest: report.derived_validation_digest.clone(),
            authority_truth_identity_digest: report
                .authority_truth_identity_digest()
                .map(str::to_string),
        }
    }

    pub(crate) fn compare_selected_equivalence(
        &self,
        other: &Self,
    ) -> TopologySelectedEquivalenceComparisonReport {
        match (
            self.selected_comparator_contract.as_ref(),
            other.selected_comparator_contract.as_ref(),
        ) {
            (Some(left), Some(right)) if left == right => {
                left.compare(&self.selected_comparable(), &other.selected_comparable())
            }
            (Some(_), Some(_)) => TopologySelectedEquivalenceComparisonReport::unsupported(
                "topology reports declared different comparator contracts",
            ),
            _ => TopologySelectedEquivalenceComparisonReport::unsupported(
                "selected equivalence family contract is required before topology comparison",
            ),
        }
    }

    fn selected_comparable(&self) -> TopologySelectedEquivalenceComparable<'_> {
        TopologySelectedEquivalenceComparable::new(
            self.selected_equivalence_family_identity,
            self.selected_equivalence_basis_identity_digest.as_deref(),
            self.selected_reuse_basis_identity_digest.as_deref(),
            self.selected_ordering_noise_posture,
            self.selected_rendered_output_comparison_posture,
            Some(&self.materialized_topology_digest),
            Some(&self.interpreted_topology_digest),
            Some(&self.derived_validation_digest),
        )
    }

    pub(crate) const fn authority_snapshot_id(&self) -> u64 {
        self.authority_snapshot_id
    }

    pub(crate) fn authority_branch_id(&self) -> &str {
        &self.authority_branch_id
    }

    pub(crate) const fn authoritative_mutation_origin(
        &self,
    ) -> schema::facade::platform::authority::MutationOrigin {
        self.authoritative_mutation_origin
    }

    pub(crate) fn truth_basis_digest_hex(&self) -> &str {
        &self.truth_basis_digest_hex
    }

    pub(crate) const fn touched_aspect_count(&self) -> usize {
        self.touched_aspect_count
    }

    pub(crate) fn triggered_invalidation_targets(
        &self,
    ) -> &[schema::facade::platform::authority::DerivedInvalidationTarget] {
        &self.triggered_invalidation_targets
    }

    pub(crate) const fn topology_compiled_product_family_identity(
        &self,
    ) -> Option<TopologyCompiledProductFamilyIdentity> {
        self.topology_compiled_product_family_identity
    }

    pub(crate) fn topology_compiled_product_family_digest(&self) -> Option<&str> {
        self.topology_compiled_product_family_digest.as_deref()
    }

    pub(crate) fn compiled_product_identity_digest(&self) -> Option<&str> {
        self.compiled_product_identity_digest.as_deref()
    }

    pub(crate) fn compiled_product_identity(&self) -> Option<&CompiledProductIdentity> {
        self.compiled_product_identity.as_ref()
    }

    pub(crate) fn equivalence_policy_identity_digest(&self) -> Option<&str> {
        self.equivalence_policy_identity_digest.as_deref()
    }

    pub(crate) fn equivalence_policy_identity(
        &self,
    ) -> Option<&CompiledProductEquivalencePolicyIdentity> {
        self.equivalence_policy_identity.as_ref()
    }

    pub(crate) fn selected_equivalence_family_identity(
        &self,
    ) -> Option<TopologySelectedEquivalenceFamilyIdentity> {
        self.selected_equivalence_family_identity
    }

    pub(crate) fn selected_equivalence_basis_identity_digest(&self) -> Option<&str> {
        self.selected_equivalence_basis_identity_digest.as_deref()
    }

    pub(crate) fn selected_compatibility_basis_identity_digest(&self) -> Option<&str> {
        self.selected_compatibility_basis_identity_digest.as_deref()
    }

    pub(crate) fn selected_reuse_basis_identity_digest(&self) -> Option<&str> {
        self.selected_reuse_basis_identity_digest.as_deref()
    }

    pub(crate) fn materialized_topology_digest(&self) -> &DeterministicDigest {
        &self.materialized_topology_digest
    }

    pub(crate) fn interpreted_topology_digest(&self) -> &DeterministicDigest {
        &self.interpreted_topology_digest
    }

    pub(crate) fn derived_validation_digest(&self) -> &DeterministicDigest {
        &self.derived_validation_digest
    }

    pub(crate) fn authority_truth_identity_digest(&self) -> Option<&str> {
        self.authority_truth_identity_digest.as_deref()
    }

    pub(crate) fn authority_truth_identity(
        &self,
    ) -> Option<&CompiledProductAuthorityTruthIdentity> {
        self.authority_truth_identity.as_ref()
    }
}
