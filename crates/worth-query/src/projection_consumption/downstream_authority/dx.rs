use crate::authorized_projection::AuthorizedProjectionArtifact;
use crate::canonicalization::CanonicalResultShapeArtifact;
use crate::query_context::QueryContextExecutionArtifact;
use crate::runtime::{
    WorthQueryDerivedArtifactBinding, WorthQueryLiveArtifactBinding, WorthQueryLiveReadResult,
    WorthQueryReadResult, WorthQueryWriteReceipt,
};

use super::{
    declared_fact_request, seal_completed_consumption, seal_completed_consumption_with_contract,
    ProjectionAuthorityContract, ProjectionAuthorityOutcome,
};
use crate::projection_consumption::{
    ProjectionFactConsumptionAttempt, ProjectionFactConsumptionPathError,
};

impl ProjectionFactConsumptionAttempt {
    pub fn into_authority(self) -> ProjectionAuthorityOutcome {
        match self {
            Self::Admitted(completed) => match seal_completed_consumption(completed) {
                Ok(authority) => ProjectionAuthorityOutcome::Admitted(Box::new(authority)),
                Err(denial) => ProjectionAuthorityOutcome::AuthorityDenied(denial),
            },
            Self::AdmittedWithWarnings(completed, warnings) => {
                match seal_completed_consumption(completed) {
                    Ok(authority) => ProjectionAuthorityOutcome::AdmittedWithWarnings(
                        Box::new(authority),
                        warnings,
                    ),
                    Err(denial) => ProjectionAuthorityOutcome::AuthorityDenied(denial),
                }
            }
            Self::Denied(denied) => ProjectionAuthorityOutcome::ConsumptionDenied(denied),
            Self::Deferred(deferred) => ProjectionAuthorityOutcome::Deferred(deferred),
            Self::SourceMismatch(mismatch) => ProjectionAuthorityOutcome::SourceMismatch(mismatch),
        }
    }

    pub fn into_authority_with_contract(
        self,
        contract: ProjectionAuthorityContract,
    ) -> ProjectionAuthorityOutcome {
        match self {
            Self::Admitted(completed) => {
                match seal_completed_consumption_with_contract(completed, contract) {
                    Ok(authority) => ProjectionAuthorityOutcome::Admitted(Box::new(authority)),
                    Err(denial) => ProjectionAuthorityOutcome::AuthorityDenied(denial),
                }
            }
            Self::AdmittedWithWarnings(completed, warnings) => {
                match seal_completed_consumption_with_contract(completed, contract) {
                    Ok(authority) => ProjectionAuthorityOutcome::AdmittedWithWarnings(
                        Box::new(authority),
                        warnings,
                    ),
                    Err(denial) => ProjectionAuthorityOutcome::AuthorityDenied(denial),
                }
            }
            Self::Denied(denied) => ProjectionAuthorityOutcome::ConsumptionDenied(denied),
            Self::Deferred(deferred) => ProjectionAuthorityOutcome::Deferred(deferred),
            Self::SourceMismatch(mismatch) => ProjectionAuthorityOutcome::SourceMismatch(mismatch),
        }
    }
}

impl WorthQueryReadResult {
    pub fn consume_projection_authority(
        &self,
        result_shape: &CanonicalResultShapeArtifact,
        authorized_projection: &AuthorizedProjectionArtifact,
        contract: ProjectionAuthorityContract,
    ) -> Result<ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError> {
        let requested = declared_fact_request(&contract);
        self.consume_projection_facts(result_shape, authorized_projection, requested)
            .map(|attempt| attempt.into_authority_with_contract(contract))
    }
}

impl WorthQueryWriteReceipt {
    pub fn consume_projection_authority(
        &self,
        result_shape_digest: &str,
        authorized_projection: &AuthorizedProjectionArtifact,
        contract: ProjectionAuthorityContract,
    ) -> Result<ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError> {
        let requested = declared_fact_request(&contract);
        self.consume_projection_facts(result_shape_digest, authorized_projection, requested)
            .map(|attempt| attempt.into_authority_with_contract(contract))
    }
}

impl QueryContextExecutionArtifact {
    pub fn consume_projection_authority(
        &self,
        authorized_projection: &AuthorizedProjectionArtifact,
        contract: ProjectionAuthorityContract,
    ) -> Result<ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError> {
        let requested = declared_fact_request(&contract);
        self.consume_projection_facts(authorized_projection, requested)
            .map(|attempt| attempt.into_authority_with_contract(contract))
    }
}

impl WorthQueryLiveReadResult {
    pub fn consume_projection_authority_with_binding(
        &self,
        binding: crate::projection_consumption::ProjectionConsumptionBindingContext,
        contract: ProjectionAuthorityContract,
    ) -> Result<ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError> {
        let requested = declared_fact_request(&contract);
        self.consume_projection_facts_with_binding(binding, requested)
            .map(|attempt| attempt.into_authority_with_contract(contract))
    }
}

impl WorthQueryDerivedArtifactBinding {
    pub fn consume_projection_authority(
        &self,
        result_shape: &CanonicalResultShapeArtifact,
        authorized_projection: &AuthorizedProjectionArtifact,
        contract: ProjectionAuthorityContract,
    ) -> Result<ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError> {
        let requested = declared_fact_request(&contract);
        self.consume_projection_facts(result_shape, authorized_projection, requested)
            .map(|attempt| attempt.into_authority_with_contract(contract))
    }
}

impl WorthQueryLiveArtifactBinding {
    pub fn consume_projection_authority(
        &self,
        result_shape_identity: &crate::evidence_identity::WorthQueryEvidenceIdentity,
        authorized_projection: &AuthorizedProjectionArtifact,
        contract: ProjectionAuthorityContract,
    ) -> Result<ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError> {
        let requested = declared_fact_request(&contract);
        self.consume_projection_facts(result_shape_identity, authorized_projection, requested)
            .map(|attempt| attempt.into_authority_with_contract(contract))
    }
}
