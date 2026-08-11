use crate::domain_computation::provider_session::graph_provider::{
    WorthQueryBoundGraphExecutionAssociation, WorthQueryCompletedDomainEvidenceDerivation,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConvergenceDomainEvidenceBindingDenial {
    StaleInstallationGeneration,
    DirectOperationRequired,
    WorkflowOperationRequired,
    StageNotInstalled,
    ArtifactContractRequired,
    ReceiptAssociationRequired,
    ExecutionAssociationMismatch,
    EmptyRunIdentity,
    EmptyCandidateSelectionKey,
}

pub(crate) struct WorthQueryConvergenceDomainEvidenceBinding {
    derivation: WorthQueryCompletedDomainEvidenceDerivation,
}

impl WorthQueryConvergenceDomainEvidenceBinding {
    pub(in crate::domain_computation) fn from_completed_execution(
        derivation: WorthQueryCompletedDomainEvidenceDerivation,
    ) -> Self {
        Self { derivation }
    }

    pub(crate) fn contract(
        &self,
    ) -> Option<&worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>
    {
        Some(self.derivation.contract())
    }

    pub(crate) fn candidate_selection_key(&self) -> &str {
        self.derivation.candidate_selection_key()
    }

    pub(crate) fn candidate_occurrence_identity(&self) -> &str {
        self.derivation.candidate_occurrence_identity()
    }

    pub(in crate::domain_computation) fn belongs_to_execution(
        &self,
        association: &WorthQueryBoundGraphExecutionAssociation,
    ) -> bool {
        self.derivation.execution() == association
    }
}
