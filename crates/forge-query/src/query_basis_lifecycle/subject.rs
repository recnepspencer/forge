use super::RawBasisIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NormalizedBasisSubject {
    CurrentHead,
    BranchHead {
        branch_identity: RawBasisIdentity,
    },
    BranchSnapshot {
        branch_identity: RawBasisIdentity,
        snapshot_identity: RawBasisIdentity,
    },
    RuntimeSnapshot {
        snapshot_identity: RawBasisIdentity,
    },
    HistoricalSnapshot {
        snapshot_identity: RawBasisIdentity,
    },
    HistoricalCommit {
        commit_identity: RawBasisIdentity,
    },
    Preview {
        preview_identity: RawBasisIdentity,
    },
    PreviewDerivedHistorical {
        preview_identity: RawBasisIdentity,
    },
}

impl NormalizedBasisSubject {
    pub fn projection_label(&self) -> String {
        match self {
            Self::CurrentHead => "current_head".to_string(),
            Self::BranchHead { branch_identity } => branch_identity.as_str().to_string(),
            Self::BranchSnapshot {
                branch_identity,
                snapshot_identity,
            } => format!(
                "{}@{}",
                branch_identity.as_str(),
                snapshot_identity.as_str()
            ),
            Self::RuntimeSnapshot { snapshot_identity }
            | Self::HistoricalSnapshot { snapshot_identity } => {
                snapshot_identity.as_str().to_string()
            }
            Self::HistoricalCommit { commit_identity } => commit_identity.as_str().to_string(),
            Self::Preview { preview_identity }
            | Self::PreviewDerivedHistorical { preview_identity } => {
                preview_identity.as_str().to_string()
            }
        }
    }
}
