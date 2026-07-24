use super::WorthQueryWorkflowStageExecutionContext;

impl WorthQueryWorkflowStageExecutionContext<'_> {
    pub fn admit_artifact_production(
        &self,
        evidence: crate::domain_installation::WorthQueryArtifactProductionEvidence,
    ) -> Result<
        crate::domain_installation::WorthQueryArtifactProductionAdmission,
        crate::domain_installation::WorthQueryArtifactDenial,
    > {
        let authority = self
            .artifact_production_authority
            .as_ref()
            .ok_or_else(|| {
            crate::domain_installation::WorthQueryArtifactDenial::new(
                crate::domain_installation::WorthQueryArtifactDenialKind::ArtifactContractNotInstalled,
                None,
                "workflow stage has no installed artifact output contract",
            )
        })?;
        Ok(
            crate::domain_installation::WorthQueryArtifactProductionAdmission::mint(
                std::sync::Arc::clone(authority),
                evidence,
            ),
        )
    }
}
