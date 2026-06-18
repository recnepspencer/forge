#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryProhibitedSeam {
    WorkspaceDirectWrite,
    WorkspaceDirectBatch,
    WorkspaceExistingTruthBindEntity,
    WorkspaceExistingTruthBindRelation,
    WorkspaceExistingTruthProbe,
    WorkspaceExistingTruthUpdate,
    WorkspaceExistingTruthAssert,
    WorkspaceExistingTruthVerify,
    WorkspaceExistingTruthUpdateVerified,
    WorkspaceExistingTruthDelete,
    WorkspaceExistingTruthDeleteWith,
    WorkspaceExistingTruthDeleteVerified,
}

impl ForgeQueryProhibitedSeam {
    pub fn key(self) -> &'static str {
        match self {
            Self::WorkspaceDirectWrite => "workspace.direct-write",
            Self::WorkspaceDirectBatch => "workspace.direct-batch",
            Self::WorkspaceExistingTruthBindEntity => "workspace.existing-truth.bind-entity",
            Self::WorkspaceExistingTruthBindRelation => "workspace.existing-truth.bind-relation",
            Self::WorkspaceExistingTruthProbe => "workspace.existing-truth.probe",
            Self::WorkspaceExistingTruthUpdate => "workspace.existing-truth.update",
            Self::WorkspaceExistingTruthAssert => "workspace.existing-truth.assert",
            Self::WorkspaceExistingTruthVerify => "workspace.existing-truth.verify",
            Self::WorkspaceExistingTruthUpdateVerified => {
                "workspace.existing-truth.update-verified"
            }
            Self::WorkspaceExistingTruthDelete => "workspace.existing-truth.delete",
            Self::WorkspaceExistingTruthDeleteWith => "workspace.existing-truth.delete-with",
            Self::WorkspaceExistingTruthDeleteVerified => {
                "workspace.existing-truth.delete-verified"
            }
        }
    }

    pub fn public_symbol(self) -> &'static str {
        match self {
            Self::WorkspaceDirectWrite => "ForgeQueryWorkspace::write",
            Self::WorkspaceDirectBatch => "ForgeQueryWorkspace::batch",
            Self::WorkspaceExistingTruthBindEntity => "ForgeQueryWorkspace::bind_existing_entity",
            Self::WorkspaceExistingTruthBindRelation => {
                "ForgeQueryWorkspace::bind_existing_relation"
            }
            Self::WorkspaceExistingTruthProbe => "ForgeQueryWorkspace::probe_existing",
            Self::WorkspaceExistingTruthUpdate => "ForgeQueryWorkspace::update_existing",
            Self::WorkspaceExistingTruthAssert => "ForgeQueryWorkspace::assert_existing",
            Self::WorkspaceExistingTruthVerify => "ForgeQueryWorkspace::verify_existing",
            Self::WorkspaceExistingTruthUpdateVerified => {
                "ForgeQueryWorkspace::update_existing_verified"
            }
            Self::WorkspaceExistingTruthDelete => "ForgeQueryWorkspace::delete_existing",
            Self::WorkspaceExistingTruthDeleteWith => "ForgeQueryWorkspace::delete_existing_with",
            Self::WorkspaceExistingTruthDeleteVerified => {
                "ForgeQueryWorkspace::delete_existing_verified"
            }
        }
    }
}
