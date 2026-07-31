use std::sync::Arc;

use worth_query_admission::facade::basis::{
    AdmittedBasisCapability, MutationPreparationLaneWitness,
};
use worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupportSnapshot;
use worth_query_installation::facade::{
    WorthQueryCompiledApplicationOperationContracts, WorthQueryInstalledGraphParticipationAuthority,
};

use super::topology::resource_topology;
use super::WorthQueryExecutionBoundOperationAuthority;
use crate::domain_computation::execution_runtime::WorthQueryExecutionRuntime;
use crate::domain_computation::operation_binding::{
    WorthQueryExecutionCommitPosture, WorthQueryInstalledDomainExecutionAuthority,
    WorthQueryInstalledOperationExecutionSupport,
};
use crate::domain_computation::provider_session::WorthQueryProviderPlanDeclarations;

pub(crate) struct WorthQueryApplicationOperationBindingInput<'a> {
    pub(crate) runtime: &'a WorthQueryExecutionRuntime,
    pub(crate) owner: &'a str,
    pub(crate) installed_operation_fingerprint: Arc<str>,
    pub(crate) resource_binding_identity: Arc<str>,
    pub(crate) basis: &'a AdmittedBasisCapability<MutationPreparationLaneWitness>,
    pub(crate) contracts: &'a WorthQueryCompiledApplicationOperationContracts,
    pub(crate) graph: &'a WorthQueryInstalledGraphParticipationAuthority,
    pub(crate) support: WorthQueryExecutionResourceSupportSnapshot,
}

impl WorthQueryExecutionBoundOperationAuthority {
    pub(crate) fn bind_application(input: WorthQueryApplicationOperationBindingInput<'_>) -> Self {
        let commit_posture = WorthQueryExecutionCommitPosture::Atomic;
        let topology = resource_topology(
            std::iter::empty(),
            &[input.graph],
            std::iter::once((
                "primary",
                worth_query_installation::facade::WorthQueryOperationGraphAccess::Project,
            )),
            std::iter::once("primary"),
            commit_posture,
        );
        Self {
            runtime_authority: input.runtime.authority_identity(),
            installation_runtime_ordinal: input.runtime.installed_packages().runtime_ordinal(),
            binding_identity: Arc::clone(&input.resource_binding_identity),
            operation_identity: Arc::clone(&input.installed_operation_fingerprint),
            basis_identity: input.basis.capability_digest().into(),
            semantic_basis: input.basis.normalized().clone(),
            canonical_query_digest: input.installed_operation_fingerprint,
            operation_resource_contract_identity: input
                .contracts
                .resources()
                .canonical_identity()
                .into(),
            provider_plan_declarations: Arc::new(
                WorthQueryProviderPlanDeclarations::from_application_contracts(input.contracts),
            ),
            commit_posture,
            direct_resource_topology: topology,
            workflow_stage_resources: None,
            operation_evidence_contract: None,
            installed_support: WorthQueryInstalledOperationExecutionSupport::direct(input.support),
            installed_domain: WorthQueryInstalledDomainExecutionAuthority::mint(
                input.runtime.authority_identity(),
                input.owner,
                input.runtime.installed_packages().generation(),
                input.runtime.retain_current_generation(),
            ),
        }
    }
}
