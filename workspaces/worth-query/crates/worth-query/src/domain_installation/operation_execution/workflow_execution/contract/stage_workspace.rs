use crate::runtime::WorthQueryWorkspace;

use super::WorthQueryWorkflowEffectEvidence;

/// The workflow-stage workspace surface. Its only executable doors live on
/// the stage context and therefore enforce declared reads and effects first.
pub struct WorthQueryWorkflowStageWorkspace<'a> {
    pub(super) workspace: &'a mut WorthQueryWorkspace,
    pub(super) artifact_registry:
        &'a crate::domain_installation::WorthQueryWorkflowArtifactRegistry,
    pub(super) artifact_production_authority:
        Option<std::sync::Arc<crate::domain_installation::WorthQueryArtifactProductionAuthority>>,
    pub(super) installed_read_executions: usize,
    pub(super) executed_effects: Vec<WorthQueryWorkflowEffectEvidence>,
}

impl<'a> WorthQueryWorkflowStageWorkspace<'a> {
    pub(crate) fn new(
        workspace: &'a mut WorthQueryWorkspace,
        artifact_registry: &'a crate::domain_installation::WorthQueryWorkflowArtifactRegistry,
        artifact_production_authority: Option<
            std::sync::Arc<crate::domain_installation::WorthQueryArtifactProductionAuthority>,
        >,
    ) -> Self {
        Self {
            workspace,
            artifact_registry,
            artifact_production_authority,
            installed_read_executions: 0,
            executed_effects: Vec::new(),
        }
    }

    pub(crate) fn installed_read_executions(&self) -> usize {
        self.installed_read_executions
    }

    pub fn register_artifact<R: crate::domain_installation::WorthQueryArtifactProviderResource>(
        &mut self,
        admission: crate::domain_installation::WorthQueryArtifactProductionAdmission,
        resource: R,
    ) -> Result<
        crate::domain_installation::WorthQueryMoveOnlyArtifactHandle,
        crate::domain_installation::WorthQueryArtifactDenial,
    > {
        let guarded = crate::domain_installation::WorthQueryGuardedArtifactResource::new(resource);
        self.register_guarded_artifact(admission, guarded)
    }

    pub(super) fn register_guarded_artifact(
        &self,
        admission: crate::domain_installation::WorthQueryArtifactProductionAdmission,
        guarded: crate::domain_installation::WorthQueryGuardedArtifactResource,
    ) -> Result<
        crate::domain_installation::WorthQueryMoveOnlyArtifactHandle,
        crate::domain_installation::WorthQueryArtifactDenial,
    > {
        let expected = self
            .artifact_production_authority
            .as_ref()
            .ok_or_else(|| {
                crate::domain_installation::WorthQueryArtifactDenial::new(
                    crate::domain_installation::WorthQueryArtifactDenialKind::ArtifactContractNotInstalled,
                    None,
                    "workflow stage has no artifact production authority",
                )
            })?;
        crate::domain_installation::WorthQueryArtifactProductionAuthority::validate_exact(
            expected,
            &admission.authority,
        )?;
        let handle = crate::domain_installation::WorthQueryMoveOnlyArtifactHandle::register(
            admission,
            guarded.prepare(),
        )?;
        self.artifact_registry.register(&handle);
        Ok(handle)
    }

    pub(crate) fn into_executed_effects(self) -> Vec<WorthQueryWorkflowEffectEvidence> {
        self.executed_effects
    }
}
