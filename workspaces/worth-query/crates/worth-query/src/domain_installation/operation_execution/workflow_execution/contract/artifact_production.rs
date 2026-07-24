use super::WorthQueryWorkflowStageExecutionContext;

impl WorthQueryWorkflowStageExecutionContext<'_> {
    pub fn admit_artifact_production(
        &self,
        evidence: crate::domain_installation::WorthQueryArtifactProductionEvidence,
    ) -> Result<
        crate::domain_installation::WorthQueryArtifactProductionAdmission,
        crate::domain_installation::WorthQueryArtifactDenial,
    > {
        let contract = self.output_artifact_contract.as_ref().ok_or_else(|| {
            crate::domain_installation::WorthQueryArtifactDenial::new(
                crate::domain_installation::WorthQueryArtifactDenialKind::ArtifactContractNotInstalled,
                None,
                "workflow stage has no installed artifact output contract",
            )
        })?;
        Ok(
            crate::domain_installation::WorthQueryArtifactProductionAdmission::mint(
                std::sync::Arc::clone(contract),
                std::sync::Arc::clone(&self.domain_authority),
                self.operation_identity.to_owned(),
                self.binding_identity.to_owned(),
                self.run_identity.to_owned(),
                self.stage.identity().to_owned(),
                self.identity_evolution_basis_identity.clone(),
                evidence,
            ),
        )
    }
}
