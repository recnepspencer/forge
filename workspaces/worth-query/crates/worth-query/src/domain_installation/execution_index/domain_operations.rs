use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryInstalledDomainOperationAuthority, WorthQueryInstalledPackageIndex,
};

use super::WorthQueryInstalledDomainArtifact;

pub(super) struct InstalledDomainOperation {
    pub(super) authority: Arc<WorthQueryInstalledDomainOperationAuthority>,
    pub(super) workflow_graph:
        Option<Arc<crate::domain_installation::WorthQueryInstalledWorkflowGraph>>,
    pub(super) evidence_contract:
        Option<Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>>,
}

pub(super) fn domain_operation_index(
    artifacts: &[WorthQueryInstalledDomainArtifact],
    portable_index: &WorthQueryInstalledPackageIndex,
) -> HashMap<(TypeId, TypeId, TypeId), InstalledDomainOperation> {
    artifacts
        .iter()
        .flat_map(|artifact| {
            artifact
                .domain_operations
                .iter()
                .map(move |operation| (artifact, operation))
        })
        .map(|(artifact, operation)| {
            let authority = portable_index
                .domain_operation(
                    &artifact.domain_owner,
                    &operation.definition().identity().slot(),
                )
                .expect("every installed domain operation has portable authority");
            portable_index
                .validate_domain_operation(&authority)
                .expect("newly minted domain-operation authority must validate");
            let evidence_contract = installed_evidence_contract(
                &artifact.domain_owner,
                &operation.definition().semantics().evidence,
                portable_index,
            );
            (
                (
                    artifact.marker_type,
                    operation.operation_marker(),
                    operation.family_marker(),
                ),
                InstalledDomainOperation {
                    workflow_graph:
                        crate::domain_installation::WorthQueryInstalledWorkflowGraph::compile(
                            authority.definition(),
                            &artifact.domain_owner,
                            portable_index,
                        )
                        .map(Arc::new),
                    evidence_contract,
                    authority: Arc::new(authority),
                },
            )
        })
        .collect()
}

fn installed_evidence_contract(
    owner: &str,
    evidence: &worth_query_installation::facade::WorthQueryDomainEvidenceContract,
    portable_index: &WorthQueryInstalledPackageIndex,
) -> Option<Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>> {
    let worth_query_installation::facade::WorthQueryDomainEvidenceContract::InstalledArtifact(
        reference,
    ) = evidence
    else {
        return None;
    };
    let authority = portable_index
        .artifact_contract(
            owner,
            reference.family().as_str(),
            reference.schema_version(),
            reference.protocol_version(),
        )
        .expect("operation evidence artifact contract must be installed");
    portable_index
        .validate_artifact_contract(&authority)
        .expect("newly minted operation evidence authority must validate");
    Some(Arc::new(authority))
}

pub(super) fn domain_operation_identity_parts(
    operations: &HashMap<(TypeId, TypeId, TypeId), InstalledDomainOperation>,
) -> Vec<String> {
    let mut identities = operations
        .values()
        .map(|operation| {
            format!(
                "{}:{}:{}",
                operation.authority.owner(),
                operation.authority.operation_slot(),
                operation.authority.definition().canonical_identity()
            )
        })
        .collect::<Vec<_>>();
    identities.sort();
    identities
}
