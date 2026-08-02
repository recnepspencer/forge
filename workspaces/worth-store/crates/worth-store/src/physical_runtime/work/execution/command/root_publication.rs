use super::types::{
    require_family, PhysicalExecutorCommand, PhysicalExecutorCommandDenial,
    PhysicalPublicationEffect, PhysicalPublicationExecutorCommand,
};
use crate::physical_runtime::work::{
    PhysicalRootPublicationWorkAction, PhysicalWorkOperationFamily,
};
use crate::physical_runtime::ResourceAdmittedPhysicalWork;

impl PhysicalExecutorCommand {
    pub(in crate::physical_runtime) fn root_publication_effect(
        work: ResourceAdmittedPhysicalWork,
    ) -> Result<Self, PhysicalExecutorCommandDenial> {
        require_family(&work, PhysicalWorkOperationFamily::RootPublication)?;
        let scope = work
            .intent()
            .scope()
            .root_publication_target()
            .ok_or(PhysicalExecutorCommandDenial::RootPublicationCommandRequiresRootScope)?;
        let (artifact, effect) = match scope.action() {
            PhysicalRootPublicationWorkAction::SynchronizeCandidateArtifact { artifact } => {
                (artifact, PhysicalPublicationEffect::SynchronizeArtifact)
            }
            PhysicalRootPublicationWorkAction::ReplaceBootstrapCatalog => (
                scope.publication().catalog_candidate(),
                PhysicalPublicationEffect::ReplaceCatalog,
            ),
            PhysicalRootPublicationWorkAction::SynchronizeParentNamespace => (
                worth_store_physical_format::RecordArtifactFile::BootstrapCatalog,
                PhysicalPublicationEffect::SynchronizeRecordFamily,
            ),
        };
        Ok(Self::RootPublicationEffect(
            PhysicalPublicationExecutorCommand {
                work,
                artifact,
                effect,
            },
        ))
    }
}
