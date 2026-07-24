use std::sync::Arc;

use super::{
    artifact_authority_denial_detail, WorthQueryArtifactAuthorityMatch, WorthQueryArtifactDenial,
    WorthQueryArtifactDenialKind,
};

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

pub(crate) struct WorthQueryArtifactProductionAuthorityParts {
    pub(crate) contract:
        Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>,
    pub(crate) domain_authority:
        Arc<crate::domain_installation::WorthQueryInstalledDomainAuthority>,
    pub(crate) operation_identity: String,
    pub(crate) binding_identity: String,
    pub(crate) run_identity: String,
    pub(crate) stage_identity: String,
    pub(crate) basis_identity: String,
}

impl WorthQueryArtifactProductionAuthority {
    pub(crate) fn mint(parts: WorthQueryArtifactProductionAuthorityParts) -> Arc<Self> {
        Arc::new(Self {
            contract: parts.contract,
            domain_authority: parts.domain_authority,
            operation_identity: parts.operation_identity,
            binding_identity: parts.binding_identity,
            run_identity: parts.run_identity,
            stage_identity: parts.stage_identity,
            basis_identity: parts.basis_identity,
        })
    }

    pub(crate) fn validate_exact(
        expected: &Arc<Self>,
        candidate: &Arc<Self>,
    ) -> Result<(), WorthQueryArtifactDenial> {
        if Arc::ptr_eq(expected, candidate) {
            return Ok(());
        }
        let kind = WorthQueryArtifactAuthorityMatch {
            runtime: expected.domain_authority.runtime_authority()
                == candidate.domain_authority.runtime_authority(),
            generation: expected.domain_authority.installation_generation()
                == candidate.domain_authority.installation_generation()
                && candidate
                    .domain_authority
                    .is_current_installation_generation(),
            operation: expected.operation_identity == candidate.operation_identity
                && expected.binding_identity == candidate.binding_identity,
            run: expected.run_identity == candidate.run_identity,
            stage: expected.stage_identity == candidate.stage_identity,
            basis: expected.basis_identity == candidate.basis_identity,
            payload_owner: expected.contract.owner() == candidate.contract.owner(),
            contract: expected.contract.contract().identity()
                == candidate.contract.contract().identity(),
        }
        .denial_kind()
        .unwrap_or(WorthQueryArtifactDenialKind::StageExecutionMismatch);
        Err(WorthQueryArtifactDenial::new(
            kind,
            Some(candidate.contract.contract().family().as_str()),
            if kind == WorthQueryArtifactDenialKind::StageExecutionMismatch {
                "artifact production admission was not minted for this stage execution"
            } else {
                artifact_authority_denial_detail(kind)
            },
        ))
    }
}
