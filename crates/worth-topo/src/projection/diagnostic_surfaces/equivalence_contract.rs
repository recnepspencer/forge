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
    authority_truth_identity: Option<CompiledProductAuthorityTruthIdentity>,
    compiled_product_identity: Option<CompiledProductIdentity>,
    equivalence_policy_identity: Option<CompiledProductEquivalencePolicyIdentity>,
    topology_compiled_product_family_identity: Option<TopologyCompiledProductFamilyIdentity>,
    topology_compiled_product_family_digest: Option<String>,
    reuse_decision_identity: Option<CompiledProductReuseDecisionIdentity>,
    pub materialized_topology_digest: DeterministicDigest,
    pub interpreted_topology_digest: DeterministicDigest,
    pub derived_validation_digest: DeterministicDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedParityComparisonReport {
    pub authority_identity_match: bool,
    pub branch_identity_match: bool,
    pub invalidation_target_match: bool,
    pub materialized_topology_digest_match: bool,
    pub interpreted_topology_digest_match: bool,
    pub derived_validation_digest_match: bool,
    pub equivalent_derived_meaning: bool,
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
        Some(selected.declaration().identity()),
        Some(&lowered),
        materialized,
        interpreted,
        validation,
    )
}

pub(crate) fn build_derived_equivalence_contract_report(
    authority_snapshot_id: u64,
    authority_branch_id: String,
    authoritative_mutation_origin: MutationOrigin,
    derivation_origin: MutationOrigin,
    truth_basis_digest_hex: String,
    touched_aspect_count: usize,
    triggered_invalidation_targets: Vec<DerivedInvalidationTarget>,
    precision_fallback_count: usize,
    precision_budget_fallback_count: usize,
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
        reuse_decision_identity: lowered_identity
            .map(|identity| identity.reuse_decision_identity().clone()),
        materialized_topology_digest: digest_materialized_topology_view(materialized),
        interpreted_topology_digest: digest_interpreted_topology_view(interpreted),
        derived_validation_digest: digest_derived_validation_report(validation),
    }
}

pub fn compare_derived_equivalence_contracts(
    lhs: &DerivedEquivalenceContractReport,
    rhs: &DerivedEquivalenceContractReport,
) -> DerivedParityComparisonReport {
    let shared_identity_basis_present =
        has_shared_identity_basis(lhs) && has_shared_identity_basis(rhs);
    let family_identity_match = lhs.topology_compiled_product_family_identity
        == rhs.topology_compiled_product_family_identity
        && lhs.topology_compiled_product_family_digest
            == rhs.topology_compiled_product_family_digest;
    let authority_identity_match = shared_identity_basis_present
        && family_identity_match
        && lhs.authority_snapshot_id == rhs.authority_snapshot_id
        && lhs.truth_basis_digest_hex == rhs.truth_basis_digest_hex
        && lhs.authority_truth_identity == rhs.authority_truth_identity
        && lhs.authoritative_mutation_origin == rhs.authoritative_mutation_origin
        && lhs.touched_aspect_count == rhs.touched_aspect_count;
    let branch_identity_match = lhs.authority_branch_id == rhs.authority_branch_id;
    let invalidation_target_match =
        lhs.triggered_invalidation_targets == rhs.triggered_invalidation_targets;
    let materialized_topology_digest_match =
        lhs.materialized_topology_digest == rhs.materialized_topology_digest;
    let interpreted_topology_digest_match =
        lhs.interpreted_topology_digest == rhs.interpreted_topology_digest;
    let derived_validation_digest_match =
        lhs.derived_validation_digest == rhs.derived_validation_digest;

    DerivedParityComparisonReport {
        authority_identity_match,
        branch_identity_match,
        invalidation_target_match,
        materialized_topology_digest_match,
        interpreted_topology_digest_match,
        derived_validation_digest_match,
        equivalent_derived_meaning: shared_identity_basis_present
            && family_identity_match
            && lhs.compiled_product_identity == rhs.compiled_product_identity
            && lhs.equivalence_policy_identity == rhs.equivalence_policy_identity
            && materialized_topology_digest_match
            && interpreted_topology_digest_match
            && derived_validation_digest_match,
    }
}

fn has_shared_identity_basis(report: &DerivedEquivalenceContractReport) -> bool {
    report.topology_compiled_product_family_identity.is_some()
        && report
            .topology_compiled_product_family_digest
            .as_ref()
            .is_some_and(|digest| !digest.trim().is_empty())
        && report.reuse_decision_identity.is_some()
        && report.authority_truth_identity.is_some()
        && report.compiled_product_identity.is_some()
        && report.equivalence_policy_identity.is_some()
}

impl DerivedEquivalenceContractReport {
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
}

#[cfg(test)]
#[path = "equivalence_contract_tests.rs"]
mod tests;
