use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactProductionEvidence {
    provenance_identity: String,
    dependency_identity: String,
}

impl WorthQueryArtifactProductionEvidence {
    pub fn new(
        provenance_identity: impl Into<String>,
        dependency_identity: impl Into<String>,
    ) -> Self {
        Self {
            provenance_identity: provenance_identity.into(),
            dependency_identity: dependency_identity.into(),
        }
    }

    pub fn provenance_identity(&self) -> &str {
        &self.provenance_identity
    }

    pub fn dependency_identity(&self) -> &str {
        &self.dependency_identity
    }

    pub(super) fn is_valid(&self) -> bool {
        [&self.provenance_identity, &self.dependency_identity]
            .into_iter()
            .all(|value| !value.trim().is_empty() && value.trim() == value)
    }
}

pub struct WorthQueryArtifactProductionAdmission {
    pub(super) contract:
        Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>,
    pub(super) domain_authority:
        Arc<crate::domain_installation::WorthQueryInstalledDomainAuthority>,
    pub(super) operation_identity: String,
    pub(super) binding_identity: String,
    pub(super) run_identity: String,
    pub(super) stage_identity: String,
    pub(super) basis_identity: String,
    pub(super) evidence: WorthQueryArtifactProductionEvidence,
}

impl WorthQueryArtifactProductionAdmission {
    pub(crate) fn mint(
        contract: Arc<
            worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority,
        >,
        domain_authority: Arc<crate::domain_installation::WorthQueryInstalledDomainAuthority>,
        operation_identity: String,
        binding_identity: String,
        run_identity: String,
        stage_identity: String,
        basis_identity: String,
        evidence: WorthQueryArtifactProductionEvidence,
    ) -> Self {
        Self {
            contract,
            domain_authority,
            operation_identity,
            binding_identity,
            run_identity,
            stage_identity,
            basis_identity,
            evidence,
        }
    }

    pub fn contract(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority {
        &self.contract
    }

    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }

    pub fn run_identity(&self) -> &str {
        &self.run_identity
    }

    pub fn stage_identity(&self) -> &str {
        &self.stage_identity
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }
}
