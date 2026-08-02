use worth_store_physical_format::RecordArtifactFile;

use crate::physical_runtime::durability::PhysicalRootPublicationIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum PhysicalRootPublicationWorkAction {
    SynchronizeCandidateArtifact { artifact: RecordArtifactFile },
    ReplaceBootstrapCatalog,
    SynchronizeParentNamespace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) struct PhysicalRootPublicationWorkScope {
    publication: PhysicalRootPublicationIdentity,
    action: PhysicalRootPublicationWorkAction,
}

impl PhysicalRootPublicationWorkScope {
    pub(in crate::physical_runtime) const fn new(
        publication: PhysicalRootPublicationIdentity,
        action: PhysicalRootPublicationWorkAction,
    ) -> Option<Self> {
        let valid = match action {
            PhysicalRootPublicationWorkAction::SynchronizeCandidateArtifact { artifact } => {
                !matches!(artifact, RecordArtifactFile::BootstrapCatalog)
            }
            PhysicalRootPublicationWorkAction::ReplaceBootstrapCatalog
            | PhysicalRootPublicationWorkAction::SynchronizeParentNamespace => true,
        };
        if valid {
            Some(Self {
                publication,
                action,
            })
        } else {
            None
        }
    }

    pub(in crate::physical_runtime) const fn publication(self) -> PhysicalRootPublicationIdentity {
        self.publication
    }

    pub(in crate::physical_runtime) const fn action(self) -> PhysicalRootPublicationWorkAction {
        self.action
    }

    pub(in crate::physical_runtime) const fn accounted_bytes(self) -> u64 {
        1
    }
}
