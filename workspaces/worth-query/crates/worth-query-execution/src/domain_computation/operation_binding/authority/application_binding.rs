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
use crate::execution_digest::hash_parts;

pub(crate) struct WorthQueryApplicationOperationBindingInput<'a> {
    pub(crate) runtime: &'a WorthQueryExecutionRuntime,
    pub(crate) owner: &'a str,
    pub(crate) installed_operation_fingerprint: &'a str,
    pub(crate) operation_scope_fingerprint: &'a [u8; 32],
    pub(crate) basis: &'a AdmittedBasisCapability<MutationPreparationLaneWitness>,
    pub(crate) contracts: &'a WorthQueryCompiledApplicationOperationContracts,
    pub(crate) graph: &'a WorthQueryInstalledGraphParticipationAuthority,
    pub(crate) support: WorthQueryExecutionResourceSupportSnapshot,
}

impl WorthQueryExecutionBoundOperationAuthority {
    pub(crate) fn bind_application(input: WorthQueryApplicationOperationBindingInput<'_>) -> Self {
        let commit_posture = WorthQueryExecutionCommitPosture::Atomic;
        let binding_identity = application_binding_identity(&input);
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
            binding_identity: binding_identity.into(),
            operation_identity: input.installed_operation_fingerprint.into(),
            basis_identity: input.basis.capability_digest().into(),
            semantic_basis: input.basis.normalized().clone(),
            canonical_query_digest: canonical_application_query_digest(&input).into(),
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

fn application_binding_identity(input: &WorthQueryApplicationOperationBindingInput<'_>) -> String {
    hash_parts(&[
        "worth_query_application_operation_binding_v1".to_owned(),
        format!("runtime:{}", input.runtime.authority_identity().as_u64()),
        format!("owner:{}", input.owner),
        format!("operation:{}", input.installed_operation_fingerprint),
        format!("scope:{}", fixed_bytes(input.operation_scope_fingerprint)),
        format!("basis:{}", input.basis.capability_digest()),
        format!("graph:{}", input.graph.authority_identity()),
        format!("support:{}", input.support.identity()),
    ])
}

fn canonical_application_query_digest(
    input: &WorthQueryApplicationOperationBindingInput<'_>,
) -> String {
    hash_parts(&[
        "worth_query_application_operation_query_v1".to_owned(),
        input.installed_operation_fingerprint.to_owned(),
    ])
}

fn fixed_bytes(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
