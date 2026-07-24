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
                    authority: Arc::new(authority),
                },
            )
        })
        .collect()
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
