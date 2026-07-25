use std::collections::BTreeMap;
use std::sync::Arc;

use worth_query_admission::facade::basis::{AdmittedBasisCapability, BasisOperationLane};
use worth_query_installation::facade::{
    WorthQueryInstalledDomainOperationAuthority, WorthQueryInstalledGraphParticipationAuthority,
    WorthQueryInstalledPackageAuthority, WorthQueryOperationWorkflowContract,
};

use super::topology::{resource_topology, touched_roles};
use super::{WorthQueryExecutionBoundOperationAuthority, WorthQueryWorkflowStageResourceAuthority};
use crate::domain_computation::artifact_owner::compile_workflow_artifact_contracts;
use crate::domain_computation::execution_runtime::{
    WorthQueryExecutionInstallationAuthority, WorthQueryExecutionRuntime,
};
use crate::domain_computation::operation_binding::{
    WorthQueryExecutionCommitPosture, WorthQueryExecutionOperationBindingDenial,
    WorthQueryInstalledOperationExecutionSupport,
};
use crate::execution_digest::hash_parts;

mod validation;

use validation::validate_operation_and_dependencies;

impl WorthQueryExecutionRuntime {
    pub fn bind_domain_operation<L: BasisOperationLane>(
        &self,
        installation_authority: &WorthQueryExecutionInstallationAuthority,
        operation: &WorthQueryInstalledDomainOperationAuthority,
        basis: &AdmittedBasisCapability<L>,
        graph_authorities: &[&WorthQueryInstalledGraphParticipationAuthority],
        required_domains: &[(&str, &WorthQueryInstalledPackageAuthority)],
        commit_posture: WorthQueryExecutionCommitPosture,
        installed_support: WorthQueryInstalledOperationExecutionSupport,
    ) -> Result<WorthQueryExecutionBoundOperationAuthority, WorthQueryExecutionOperationBindingDenial>
    {
        let workflow_artifact_contracts = match &operation.definition().semantics().workflow {
            WorthQueryOperationWorkflowContract::Declared(workflow) => {
                compile_workflow_artifact_contracts(
                    operation.owner(),
                    workflow.stages(),
                    self.installed_packages(),
                )
            }
            WorthQueryOperationWorkflowContract::NotRequired => BTreeMap::new(),
        };
        validate_operation_and_dependencies(
            self,
            installation_authority,
            operation,
            graph_authorities,
            required_domains,
            commit_posture,
            &installed_support,
            &workflow_artifact_contracts,
        )?;
        Ok(WorthQueryExecutionBoundOperationAuthority {
            runtime_authority: self.authority_identity(),
            installation_runtime_ordinal: operation.runtime_ordinal(),
            binding_identity: binding_identity(
                self,
                operation,
                basis,
                graph_authorities,
                required_domains,
                commit_posture,
                &installed_support,
            )
            .into(),
            operation_identity: operation.definition().canonical_identity().into(),
            basis_identity: basis.capability_digest().into(),
            canonical_query_digest: operation
                .definition()
                .semantics()
                .canonical_query
                .query()
                .digest()
                .as_str()
                .into(),
            operation_resource_contract_identity: operation
                .definition()
                .semantics()
                .resources
                .canonical_identity()
                .into(),
            commit_posture,
            direct_resource_topology: direct_topology(operation, graph_authorities, commit_posture),
            workflow_stage_resources: workflow_stage_resources(
                operation,
                graph_authorities,
                commit_posture,
                &workflow_artifact_contracts,
            ),
            operation_evidence_contract: installed_evidence_contract(
                operation.owner(),
                &operation.definition().semantics().evidence,
                self.installed_packages(),
            ),
            installed_support,
            installed_domain:
                crate::domain_computation::operation_binding::WorthQueryInstalledDomainExecutionAuthority::mint(
                    self.authority_identity(),
                    operation.owner(),
                    operation.generation(),
                    self.retain_current_generation(),
                ),
        })
    }
}

fn direct_topology(
    operation: &WorthQueryInstalledDomainOperationAuthority,
    graph_authorities: &[&WorthQueryInstalledGraphParticipationAuthority],
    commit_posture: WorthQueryExecutionCommitPosture,
) -> super::WorthQueryExecutionResourceTopology {
    let semantics = operation.definition().semantics();
    resource_topology(
        semantics
            .conditional_nodes
            .iter()
            .map(|node| format!("operation:{}", node.identity())),
        graph_authorities,
        semantics.graph_reads.roles().iter().filter_map(|read| {
            graph_authorities
                .iter()
                .any(|authority| authority.role() == read.role)
                .then_some((read.role.as_str(), read.access))
        }),
        touched_roles(semantics).into_iter(),
        commit_posture,
    )
}

fn workflow_stage_resources(
    operation: &WorthQueryInstalledDomainOperationAuthority,
    graph_authorities: &[&WorthQueryInstalledGraphParticipationAuthority],
    commit_posture: WorthQueryExecutionCommitPosture,
    artifact_contracts: &BTreeMap<
        String,
        crate::domain_computation::artifact_owner::WorthQueryInstalledWorkflowArtifactContracts,
    >,
) -> Option<BTreeMap<Arc<str>, WorthQueryWorkflowStageResourceAuthority>> {
    let WorthQueryOperationWorkflowContract::Declared(workflow) =
        &operation.definition().semantics().workflow
    else {
        return None;
    };
    Some(
        workflow
            .stages()
            .iter()
            .map(|stage| {
                (
                    Arc::<str>::from(stage.identity()),
                    WorthQueryWorkflowStageResourceAuthority {
                        contract_identity: stage.semantics().resources.canonical_identity().into(),
                        topology: resource_topology(
                            stage.semantics().conditional_nodes.iter().map(|node| {
                                format!("stage:{}:{}", stage.identity(), node.identity())
                            }),
                            graph_authorities,
                            stage
                                .semantics()
                                .graph_read_roles
                                .iter()
                                .filter_map(|role| {
                                    operation
                                        .definition()
                                        .semantics()
                                        .graph_reads
                                        .roles()
                                        .iter()
                                        .find(|read| read.role == *role)
                                        .map(|read| (role.as_str(), read.access))
                                }),
                            stage.semantics().touch_roles.iter().map(String::as_str),
                            commit_posture,
                        ),
                        predecessors: stage.predecessors().to_vec().into(),
                        artifact_contracts: artifact_contracts
                            .get(stage.identity())
                            .expect("installed workflow stage must retain artifact contracts")
                            .clone(),
                    },
                )
            })
            .collect(),
    )
}

fn installed_evidence_contract(
    owner: &str,
    evidence: &worth_query_installation::facade::WorthQueryDomainEvidenceContract,
    installed: &worth_query_installation::facade::WorthQueryInstalledPackageIndex,
) -> Option<Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>> {
    let reference = evidence.artifact_reference()?;
    let authority = installed
        .artifact_contract(
            owner,
            reference.family().as_str(),
            reference.schema_version(),
            reference.protocol_version(),
        )
        .expect("operation evidence artifact contract must be installed");
    installed
        .validate_artifact_contract(&authority)
        .expect("operation evidence artifact authority must validate");
    Some(Arc::new(authority))
}

fn binding_identity<L: BasisOperationLane>(
    runtime: &WorthQueryExecutionRuntime,
    operation: &WorthQueryInstalledDomainOperationAuthority,
    basis: &AdmittedBasisCapability<L>,
    graph_authorities: &[&WorthQueryInstalledGraphParticipationAuthority],
    required_domains: &[(&str, &WorthQueryInstalledPackageAuthority)],
    commit_posture: WorthQueryExecutionCommitPosture,
    installed_support: &WorthQueryInstalledOperationExecutionSupport,
) -> String {
    let mut graphs = graph_authorities
        .iter()
        .map(|authority| authority.authority_identity())
        .collect::<Vec<_>>();
    graphs.sort_unstable();
    let mut domains = required_domains
        .iter()
        .map(|(role, authority)| format!("{role}:{}", authority.admission_identity()))
        .collect::<Vec<_>>();
    domains.sort_unstable();
    hash_parts(&[
        "worth_query_execution_bound_operation_v1".into(),
        format!("runtime:{}", runtime.authority_identity().as_u64()),
        format!("operation:{}", operation.definition().canonical_identity()),
        format!("basis:{}", basis.capability_digest()),
        format!("graphs:{}", graphs.join(",")),
        format!("required-domains:{}", domains.join(",")),
        format!("commit-posture:{}", commit_posture.as_str()),
        format!("installed-support:{}", installed_support.identity()),
    ])
}
