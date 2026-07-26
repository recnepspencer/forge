use std::sync::Arc;

use worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority;

use super::operation_binding::{
    WorthQueryExecutionBoundOperationAuthority, WorthQueryInstalledDomainExecutionAuthority,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainEvidenceBindingDenial {
    StaleInstallationGeneration,
    DirectOperationRequired,
    WorkflowOperationRequired,
    StageNotInstalled,
    EmptyRunIdentity,
    EmptyExecutionSnapshotIdentity,
    EmptyOutputOccurrenceIdentity,
}

pub struct WorthQueryDomainEvidenceExecutionBinding {
    contract: Option<Arc<WorthQueryInstalledArtifactContractAuthority>>,
    domain_authority: Arc<WorthQueryInstalledDomainExecutionAuthority>,
    operation_identity: Arc<str>,
    binding_identity: Arc<str>,
    run_identity: Option<Arc<str>>,
    stage_identity: Option<Arc<str>>,
    basis_identity: Arc<str>,
    execution_snapshot_identity: String,
    output_occurrence_identity: String,
}

impl WorthQueryDomainEvidenceExecutionBinding {
    pub(crate) fn direct(
        authority: &WorthQueryExecutionBoundOperationAuthority,
        execution_snapshot_identity: &str,
        output_occurrence_identity: &str,
    ) -> Result<Self, WorthQueryDomainEvidenceBindingDenial> {
        if authority.is_workflow_operation() {
            return Err(WorthQueryDomainEvidenceBindingDenial::DirectOperationRequired);
        }
        Self::mint(
            authority.operation_evidence_contract().cloned(),
            authority,
            None,
            None,
            execution_snapshot_identity,
            output_occurrence_identity,
        )
    }

    pub(crate) fn workflow_stage(
        authority: &WorthQueryExecutionBoundOperationAuthority,
        run_identity: &str,
        stage_identity: &str,
        execution_snapshot_identity: &str,
        output_occurrence_identity: &str,
    ) -> Result<Self, WorthQueryDomainEvidenceBindingDenial> {
        if !authority.is_workflow_operation() {
            return Err(WorthQueryDomainEvidenceBindingDenial::WorkflowOperationRequired);
        }
        if run_identity.trim().is_empty() {
            return Err(WorthQueryDomainEvidenceBindingDenial::EmptyRunIdentity);
        }
        let contracts = authority
            .workflow_stage_artifact_contracts(stage_identity)
            .ok_or(WorthQueryDomainEvidenceBindingDenial::StageNotInstalled)?;
        Self::mint(
            contracts.evidence().cloned(),
            authority,
            Some(Arc::from(run_identity)),
            Some(Arc::from(stage_identity)),
            execution_snapshot_identity,
            output_occurrence_identity,
        )
    }

    fn mint(
        contract: Option<Arc<WorthQueryInstalledArtifactContractAuthority>>,
        authority: &WorthQueryExecutionBoundOperationAuthority,
        run_identity: Option<Arc<str>>,
        stage_identity: Option<Arc<str>>,
        execution_snapshot_identity: &str,
        output_occurrence_identity: &str,
    ) -> Result<Self, WorthQueryDomainEvidenceBindingDenial> {
        if !authority.is_current_installation_generation() {
            return Err(WorthQueryDomainEvidenceBindingDenial::StaleInstallationGeneration);
        }
        if execution_snapshot_identity.trim().is_empty() {
            return Err(WorthQueryDomainEvidenceBindingDenial::EmptyExecutionSnapshotIdentity);
        }
        if output_occurrence_identity.trim().is_empty() {
            return Err(WorthQueryDomainEvidenceBindingDenial::EmptyOutputOccurrenceIdentity);
        }
        Ok(Self {
            contract,
            domain_authority: authority.retain_installed_domain_authority(),
            operation_identity: Arc::from(authority.operation_identity()),
            binding_identity: Arc::from(authority.binding_identity()),
            run_identity,
            stage_identity,
            basis_identity: Arc::from(authority.basis_identity()),
            execution_snapshot_identity: execution_snapshot_identity.to_owned(),
            output_occurrence_identity: output_occurrence_identity.to_owned(),
        })
    }

    pub fn contract(&self) -> Option<&WorthQueryInstalledArtifactContractAuthority> {
        self.contract.as_deref()
    }

    pub fn is_current_installation_generation(&self) -> bool {
        self.domain_authority.is_current_installation_generation()
    }

    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn run_identity(&self) -> Option<&str> {
        self.run_identity.as_deref()
    }

    pub fn stage_identity(&self) -> Option<&str> {
        self.stage_identity.as_deref()
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn execution_snapshot_identity(&self) -> &str {
        &self.execution_snapshot_identity
    }

    pub fn output_occurrence_identity(&self) -> &str {
        &self.output_occurrence_identity
    }
}
