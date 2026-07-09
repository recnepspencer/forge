#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryProhibitedSeam {
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

impl WorthQueryProhibitedSeam {
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
            Self::WorkspaceDirectWrite => "WorthQueryWorkspace::write",
            Self::WorkspaceDirectBatch => "WorthQueryWorkspace::batch",
            Self::WorkspaceExistingTruthBindEntity => "WorthQueryWorkspace::bind_existing_entity",
            Self::WorkspaceExistingTruthBindRelation => {
                "WorthQueryWorkspace::bind_existing_relation"
            }
            Self::WorkspaceExistingTruthProbe => "WorthQueryWorkspace::probe_existing",
            Self::WorkspaceExistingTruthUpdate => "WorthQueryWorkspace::update_existing",
            Self::WorkspaceExistingTruthAssert => "WorthQueryWorkspace::assert_existing",
            Self::WorkspaceExistingTruthVerify => "WorthQueryWorkspace::verify_existing",
            Self::WorkspaceExistingTruthUpdateVerified => {
                "WorthQueryWorkspace::update_existing_verified"
            }
            Self::WorkspaceExistingTruthDelete => "WorthQueryWorkspace::delete_existing",
            Self::WorkspaceExistingTruthDeleteWith => "WorthQueryWorkspace::delete_existing_with",
            Self::WorkspaceExistingTruthDeleteVerified => {
                "WorthQueryWorkspace::delete_existing_verified"
            }
        }
    }
}
