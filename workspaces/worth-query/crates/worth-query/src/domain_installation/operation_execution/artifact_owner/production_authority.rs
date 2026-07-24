use std::sync::Arc;

use super::{WorthQueryArtifactDenial, WorthQueryArtifactDenialKind};

pub(crate) struct WorthQueryArtifactProductionAuthority {
    pub(super) contract:
        Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>,
    pub(super) domain_authority:
        Arc<crate::domain_installation::WorthQueryInstalledDomainAuthority>,
    pub(super) operation_identity: String,
    pub(super) binding_identity: String,
    pub(super) run_identity: String,
    pub(super) stage_identity: String,
    pub(super) basis_identity: String,
}

impl WorthQueryArtifactProductionAuthority {
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
    ) -> Arc<Self> {
        Arc::new(Self {
            contract,
            domain_authority,
            operation_identity,
            binding_identity,
            run_identity,
            stage_identity,
            basis_identity,
        })
    }

    pub(crate) fn validate_exact(
        expected: &Arc<Self>,
        candidate: &Arc<Self>,
    ) -> Result<(), WorthQueryArtifactDenial> {
        if Arc::ptr_eq(expected, candidate) {
            return Ok(());
        }
        let kind = if expected.domain_authority.runtime_authority()
            != candidate.domain_authority.runtime_authority()
        {
            WorthQueryArtifactDenialKind::ForeignRuntime
        } else if expected.domain_authority.installation_generation()
            != candidate.domain_authority.installation_generation()
            || !candidate
                .domain_authority
                .is_current_installation_generation()
        {
            WorthQueryArtifactDenialKind::StaleInstallationGeneration
        } else if expected.operation_identity != candidate.operation_identity
            || expected.binding_identity != candidate.binding_identity
        {
            WorthQueryArtifactDenialKind::OperationMismatch
        } else if expected.run_identity != candidate.run_identity {
            WorthQueryArtifactDenialKind::RunMismatch
        } else if expected.stage_identity != candidate.stage_identity {
            WorthQueryArtifactDenialKind::StageMismatch
        } else if expected.basis_identity != candidate.basis_identity {
            WorthQueryArtifactDenialKind::BasisMismatch
        } else if expected.contract.owner() != candidate.contract.owner() {
            WorthQueryArtifactDenialKind::PayloadOwnerMismatch
        } else if expected.contract.contract().identity()
            != candidate.contract.contract().identity()
        {
            WorthQueryArtifactDenialKind::ArtifactContractMismatch
        } else {
            WorthQueryArtifactDenialKind::StageExecutionMismatch
        };
        Err(WorthQueryArtifactDenial::new(
            kind,
            Some(candidate.contract.contract().family().as_str()),
            production_authority_denial_detail(kind),
        ))
    }
}

fn production_authority_denial_detail(kind: WorthQueryArtifactDenialKind) -> &'static str {
    match kind {
        WorthQueryArtifactDenialKind::ForeignRuntime => {
            "artifact production admission belongs to a different Query runtime"
        }
        WorthQueryArtifactDenialKind::StaleInstallationGeneration => {
            "artifact production admission belongs to a stale installation generation"
        }
        WorthQueryArtifactDenialKind::OperationMismatch => {
            "artifact production admission belongs to a different operation binding"
        }
        WorthQueryArtifactDenialKind::RunMismatch => {
            "artifact production admission belongs to a different workflow run"
        }
        WorthQueryArtifactDenialKind::StageMismatch => {
            "artifact production admission belongs to a different workflow stage"
        }
        WorthQueryArtifactDenialKind::BasisMismatch => {
            "artifact production admission belongs to a different admitted basis"
        }
        WorthQueryArtifactDenialKind::PayloadOwnerMismatch => {
            "artifact production admission belongs to a different payload owner"
        }
        WorthQueryArtifactDenialKind::ArtifactContractMismatch => {
            "artifact production admission names a different installed artifact contract"
        }
        WorthQueryArtifactDenialKind::StageExecutionMismatch => {
            "artifact production admission was not minted for this stage execution"
        }
        _ => "artifact production admission does not match this stage execution",
    }
}
