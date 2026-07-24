use super::{WorthQueryBoundGraphExecutionReceipt, WorthQueryWorkflowStageReceipt};

pub(crate) struct WorthQueryWorkflowStageExecutionAuthority<'a> {
    pub(crate) effect_workflow_binding: crate::workflow::WorkflowContextBinding,
    pub(crate) basis: crate::basis_lifecycle::BasisFamily,
    pub(crate) installed_read: Option<&'a crate::ordinary::read::WorthQueryReadDeclaration>,
    pub(crate) operation_graph_reads:
        &'a [worth_query_installation::facade::WorthQueryOperationGraphReadRole],
    pub(crate) graph_receipts: &'a [WorthQueryBoundGraphExecutionReceipt],
    pub(crate) query_authority: crate::identity_authority::QueryCanonicalAuthority,
    pub(crate) identity_evolution_basis_identity: String,
    pub(crate) domain_authority:
        std::sync::Arc<crate::domain_installation::WorthQueryInstalledDomainAuthority>,
    pub(crate) output_artifact_contract: Option<
        std::sync::Arc<
            worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority,
        >,
    >,
}

pub(crate) struct WorthQueryWorkflowStageExecutionScope<'a> {
    pub(crate) operation_identity: &'a str,
    pub(crate) binding_identity: &'a str,
    pub(crate) run_identity: &'a str,
    pub(crate) stage: &'a worth_query_installation::facade::WorthQueryPortableWorkflowStage,
    pub(crate) predecessor_receipts: &'a [&'a WorthQueryWorkflowStageReceipt],
}
