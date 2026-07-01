use schema::facade::platform::authority::compiled_product_semantic_graph::{
    CompiledProductAuthorityTruthIdentity, CompiledProductEquivalencePolicyIdentity,
    CompiledProductIdentity, CompiledProductReuseDecisionIdentity,
};
use schema::facade::platform::authority::{DerivedInvalidationTarget, MutationOrigin};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use serde::{Deserialize, Serialize};

use crate::compiled_product_family::{
    current_topology_compiled_product_family_catalog,
    digest_derived_validation_report as family_digest_derived_validation_report,
    digest_interpreted_topology_view as family_digest_interpreted_topology_view,
    digest_materialized_topology_view as family_digest_materialized_topology_view,
    select_topology_compiled_product_family, DeterministicDigest, TopologyCompiledProductConsumer,
    TopologyCompiledProductFamilyIdentity, TopologyCompiledProductLoweredIdentity,
};
use crate::derived_invalidation_compiled_product_admission::{
    admit_topology_compiled_product_input, TopologyCompiledProductAdmissionRequest,
};
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::selected_equivalence_family::{
    current_topology_selected_equivalence_family_catalog, select_topology_equivalence_family,
    SelectedTopologyEquivalenceFamily, TopologyCompatibilityPosture,
    TopologyFreshnessRequirementPosture, TopologyOrderingNoisePosture,
    TopologyRenderedOutputComparisonPosture, TopologySelectedEquivalenceComparatorContract,
    TopologySelectedEquivalenceFamilyIdentity,
};
use crate::validation::DerivedTopologyValidationReport;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedEquivalenceContractReport {
    pub authority_snapshot_id: u64,
    pub authority_branch_id: String,
    pub authoritative_mutation_origin: MutationOrigin,
    pub derivation_origin: MutationOrigin,
    pub truth_basis_digest_hex: String,
    pub touched_aspect_count: usize,
    pub triggered_invalidation_targets: Vec<DerivedInvalidationTarget>,
    pub precision_fallback_count: usize,
    pub precision_budget_fallback_count: usize,
    pub(crate) authority_truth_identity: Option<CompiledProductAuthorityTruthIdentity>,
    pub(crate) compiled_product_identity: Option<CompiledProductIdentity>,
    pub(crate) equivalence_policy_identity: Option<CompiledProductEquivalencePolicyIdentity>,
    pub(crate) topology_compiled_product_family_identity:
        Option<TopologyCompiledProductFamilyIdentity>,
    pub(crate) topology_compiled_product_family_digest: Option<String>,
    pub(crate) selected_equivalence_family_identity:
        Option<TopologySelectedEquivalenceFamilyIdentity>,
    pub(crate) selected_equivalence_basis_identity_digest: Option<String>,
    pub(crate) selected_compatibility_basis_identity_digest: Option<String>,
    pub(crate) selected_reuse_basis_identity_digest: Option<String>,
    pub(crate) future_public_proof_seed_identity_digest: Option<String>,
    pub(crate) selected_comparator_contract: Option<TopologySelectedEquivalenceComparatorContract>,
    pub(crate) selected_compatibility_posture: Option<TopologyCompatibilityPosture>,
    pub(crate) selected_freshness_requirement_posture: Option<TopologyFreshnessRequirementPosture>,
    pub(crate) selected_ordering_noise_posture: Option<TopologyOrderingNoisePosture>,
    pub(crate) selected_rendered_output_comparison_posture:
        Option<TopologyRenderedOutputComparisonPosture>,
    pub(crate) reuse_decision_identity: Option<CompiledProductReuseDecisionIdentity>,
    pub materialized_topology_digest: DeterministicDigest,
    pub interpreted_topology_digest: DeterministicDigest,
    pub derived_validation_digest: DeterministicDigest,
}

pub fn digest_materialized_topology_view(
    materialized: &MaterializedTopologyView,
) -> DeterministicDigest {
    family_digest_materialized_topology_view(materialized)
}

pub fn digest_interpreted_topology_view(
    interpreted: &InterpretedTopologyView,
) -> DeterministicDigest {
    family_digest_interpreted_topology_view(interpreted)
}

pub fn digest_derived_validation_report(
    validation: &DerivedTopologyValidationReport,
) -> DeterministicDigest {
    family_digest_derived_validation_report(validation)
}

pub fn build_derived_equivalence_contract(
    read_basis: &DerivedTopologyReadBasis,
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
    validation: &DerivedTopologyValidationReport,
) -> DerivedEquivalenceContractReport {
    let catalog = current_topology_compiled_product_family_catalog();
    let admitted = admit_topology_compiled_product_input(
        &catalog,
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            read_basis,
        ),
    )
    .expect("derived topology compiled-product admission");
    let selected_equivalence_family = select_topology_equivalence_family(
        &current_topology_selected_equivalence_family_catalog(),
        &admitted,
    )
    .expect("selected topology equivalence family");
    let selected = select_topology_compiled_product_family(
        &catalog,
        admitted.clone().into_family_admitted_input(),
    )
    .expect("derived topology compiled-product family selection");
    let lowered = selected
        .compile_product_identity(materialized, interpreted, validation)
        .expect("derived topology compiled-product family lowering");

    build_derived_equivalence_contract_report(
        admitted.source_authority_basis().authority_snapshot_id(),
        admitted
            .source_authority_basis()
            .authority_branch_id()
            .to_string(),
        read_basis.authoritative_mutation_origin(),
        read_basis.derivation_origin(),
        admitted
            .source_authority_basis()
            .truth_basis_digest_hex()
            .to_string(),
        admitted.source_authority_basis().touched_aspect_count(),
        admitted
            .locality_basis()
            .triggered_invalidation_targets()
            .to_vec(),
        admitted.source_authority_basis().precision_fallback_count(),
        admitted
            .source_authority_basis()
            .precision_budget_fallback_count(),
        Some(&selected_equivalence_family),
        Some(selected.declaration().identity()),
        Some(&lowered),
        materialized,
        interpreted,
        validation,
    )
}

pub fn build_derived_equivalence_contract_report(
    authority_snapshot_id: u64,
    authority_branch_id: String,
    authoritative_mutation_origin: MutationOrigin,
    derivation_origin: MutationOrigin,
    truth_basis_digest_hex: String,
    touched_aspect_count: usize,
    triggered_invalidation_targets: Vec<DerivedInvalidationTarget>,
    precision_fallback_count: usize,
    precision_budget_fallback_count: usize,
    selected_equivalence_family: Option<&SelectedTopologyEquivalenceFamily>,
    topology_compiled_product_family_identity: Option<TopologyCompiledProductFamilyIdentity>,
    lowered_identity: Option<&TopologyCompiledProductLoweredIdentity>,
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
    validation: &DerivedTopologyValidationReport,
) -> DerivedEquivalenceContractReport {
    DerivedEquivalenceContractReport {
        authority_snapshot_id,
        authority_branch_id,
        authoritative_mutation_origin,
        derivation_origin,
        truth_basis_digest_hex,
        touched_aspect_count,
        triggered_invalidation_targets,
        precision_fallback_count,
        precision_budget_fallback_count,
        authority_truth_identity: lowered_identity
            .map(|identity| identity.authority_truth_identity().clone()),
        compiled_product_identity: lowered_identity
            .map(|identity| identity.compiled_product_identity().clone()),
        equivalence_policy_identity: lowered_identity
            .map(|identity| identity.equivalence_policy_identity().clone()),
        topology_compiled_product_family_identity,
        topology_compiled_product_family_digest: lowered_identity
            .map(|identity| identity.family_digest().to_string()),
        selected_equivalence_family_identity: selected_equivalence_family
            .map(SelectedTopologyEquivalenceFamily::family_identity),
        selected_equivalence_basis_identity_digest: selected_equivalence_family.map(|family| {
            family
                .equivalence_basis_identity()
                .identity_digest()
                .to_string()
        }),
        selected_compatibility_basis_identity_digest: selected_equivalence_family.map(|family| {
            family
                .compatibility_basis_identity()
                .identity_digest()
                .to_string()
        }),
        selected_reuse_basis_identity_digest: selected_equivalence_family
            .map(|family| family.reuse_basis_identity().identity_digest().to_string()),
        future_public_proof_seed_identity_digest: selected_equivalence_family.map(|family| {
            family
                .future_public_proof_seed_identity()
                .identity_digest()
                .to_string()
        }),
        selected_comparator_contract: selected_equivalence_family
            .map(SelectedTopologyEquivalenceFamily::comparator_contract),
        selected_compatibility_posture: selected_equivalence_family
            .map(SelectedTopologyEquivalenceFamily::compatibility_posture),
        selected_freshness_requirement_posture: selected_equivalence_family
            .map(SelectedTopologyEquivalenceFamily::freshness_requirement_posture),
        selected_ordering_noise_posture: selected_equivalence_family
            .map(SelectedTopologyEquivalenceFamily::ordering_noise_posture),
        selected_rendered_output_comparison_posture: selected_equivalence_family
            .map(SelectedTopologyEquivalenceFamily::rendered_output_comparison_posture),
        reuse_decision_identity: lowered_identity
            .map(|identity| identity.reuse_decision_identity().clone()),
        materialized_topology_digest: digest_materialized_topology_view(materialized),
        interpreted_topology_digest: digest_interpreted_topology_view(interpreted),
        derived_validation_digest: digest_derived_validation_report(validation),
    }
}

impl DerivedEquivalenceContractReport {
    pub(crate) fn rebuild_required_identity(
        &self,
        compiled_product_identity: &CompiledProductIdentity,
        denial_reason: &str,
    ) -> schema::facade::platform::authority::compiled_product_semantic_graph::CompiledProductRebuildDenialIdentity{
        schema::facade::platform::authority::compiled_product_semantic_graph::admit_compiled_product_rebuild_denial_identity(
            compiled_product_identity,
            denial_reason,
        )
        .expect("topology derived rebuild denial identity")
    }

    pub(crate) fn compiled_product_identity_ref(&self) -> Option<&CompiledProductIdentity> {
        self.compiled_product_identity.as_ref()
    }

    pub(crate) fn authority_truth_identity_ref(
        &self,
    ) -> Option<&CompiledProductAuthorityTruthIdentity> {
        self.authority_truth_identity.as_ref()
    }

    pub(crate) fn equivalence_policy_identity_ref(
        &self,
    ) -> Option<&CompiledProductEquivalencePolicyIdentity> {
        self.equivalence_policy_identity.as_ref()
    }

    pub(crate) fn reuse_decision_identity_ref(
        &self,
    ) -> Option<&CompiledProductReuseDecisionIdentity> {
        self.reuse_decision_identity.as_ref()
    }

    pub(crate) fn selected_comparator_contract(
        &self,
    ) -> Option<TopologySelectedEquivalenceComparatorContract> {
        self.selected_comparator_contract.clone()
    }

    pub(crate) fn selected_ordering_noise_posture(&self) -> Option<TopologyOrderingNoisePosture> {
        self.selected_ordering_noise_posture
    }

    pub(crate) fn selected_rendered_output_comparison_posture(
        &self,
    ) -> Option<TopologyRenderedOutputComparisonPosture> {
        self.selected_rendered_output_comparison_posture
    }

    pub fn authority_truth_identity_digest(&self) -> Option<&str> {
        self.authority_truth_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }

    pub fn compiled_product_identity_digest(&self) -> Option<&str> {
        self.compiled_product_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }

    pub fn equivalence_policy_identity_digest(&self) -> Option<&str> {
        self.equivalence_policy_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }

    pub fn topology_compiled_product_family_identity(
        &self,
    ) -> Option<TopologyCompiledProductFamilyIdentity> {
        self.topology_compiled_product_family_identity
    }

    pub fn topology_compiled_product_family_digest(&self) -> Option<&str> {
        self.topology_compiled_product_family_digest.as_deref()
    }

    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.reuse_decision_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }

    pub fn selected_equivalence_family_identity(
        &self,
    ) -> Option<TopologySelectedEquivalenceFamilyIdentity> {
        self.selected_equivalence_family_identity
    }

    pub fn selected_equivalence_basis_identity_digest(&self) -> Option<&str> {
        self.selected_equivalence_basis_identity_digest.as_deref()
    }

    pub fn selected_compatibility_basis_identity_digest(&self) -> Option<&str> {
        self.selected_compatibility_basis_identity_digest.as_deref()
    }

    pub fn selected_reuse_basis_identity_digest(&self) -> Option<&str> {
        self.selected_reuse_basis_identity_digest.as_deref()
    }

    #[cfg(test)]
    pub(crate) fn with_test_selected_family_contract_removed(mut self) -> Self {
        self.selected_equivalence_family_identity = None;
        self.selected_equivalence_basis_identity_digest = None;
        self.selected_compatibility_basis_identity_digest = None;
        self.selected_reuse_basis_identity_digest = None;
        self.future_public_proof_seed_identity_digest = None;
        self.selected_comparator_contract = None;
        self.selected_compatibility_posture = None;
        self.selected_freshness_requirement_posture = None;
        self.selected_ordering_noise_posture = None;
        self.selected_rendered_output_comparison_posture = None;
        self
    }
}
