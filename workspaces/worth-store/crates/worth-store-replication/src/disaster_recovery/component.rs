use std::path::{Component, Path, PathBuf};

use super::DisasterRecoveryBundleDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DisasterRecoveryComponentFamily {
    Authority,
    Checkpoint,
    Wal,
    Page,
    Blob,
    Layout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisasterRecoveryArtifactEvidence {
    expected_digest: [u8; 32],
    byte_length: u64,
    format_identity: [u8; 32],
    backend_assumption_identity: [u8; 32],
}

impl DisasterRecoveryArtifactEvidence {
    pub fn admit(
        expected_digest: [u8; 32],
        byte_length: u64,
        format_identity: [u8; 32],
        backend_assumption_identity: [u8; 32],
    ) -> Result<Self, DisasterRecoveryBundleDenial> {
        if expected_digest == [0; 32]
            || byte_length == 0
            || format_identity == [0; 32]
            || backend_assumption_identity == [0; 32]
        {
            return Err(DisasterRecoveryBundleDenial::InvalidComponent);
        }
        Ok(Self {
            expected_digest,
            byte_length,
            format_identity,
            backend_assumption_identity,
        })
    }

    pub const fn expected_digest(self) -> [u8; 32] {
        self.expected_digest
    }

    pub const fn byte_length(self) -> u64 {
        self.byte_length
    }

    pub const fn format_identity(self) -> [u8; 32] {
        self.format_identity
    }

    pub const fn backend_assumption_identity(self) -> [u8; 32] {
        self.backend_assumption_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisasterRecoveryComponentSemantics {
    Authority {
        lineage_identity: [u8; 32],
        authority_epoch: u64,
    },
    Checkpoint {
        lineage_identity: [u8; 32],
        authority_epoch: u64,
        checkpoint_identity: [u8; 32],
        checkpoint_lsn: u64,
        blob_closure_identity: [u8; 32],
    },
    Wal {
        lineage_identity: [u8; 32],
        authority_epoch: u64,
        start_lsn: u64,
        end_lsn_exclusive: u64,
    },
    Page {
        checkpoint_identity: [u8; 32],
    },
    Blob {
        blob_closure_identity: [u8; 32],
    },
    Layout {
        checkpoint_identity: [u8; 32],
    },
}

impl DisasterRecoveryComponentSemantics {
    pub const fn family(self) -> DisasterRecoveryComponentFamily {
        match self {
            Self::Authority { .. } => DisasterRecoveryComponentFamily::Authority,
            Self::Checkpoint { .. } => DisasterRecoveryComponentFamily::Checkpoint,
            Self::Wal { .. } => DisasterRecoveryComponentFamily::Wal,
            Self::Page { .. } => DisasterRecoveryComponentFamily::Page,
            Self::Blob { .. } => DisasterRecoveryComponentFamily::Blob,
            Self::Layout { .. } => DisasterRecoveryComponentFamily::Layout,
        }
    }

    pub(super) fn is_structurally_valid(self) -> bool {
        match self {
            Self::Authority {
                lineage_identity,
                authority_epoch,
            } => lineage_identity != [0; 32] && authority_epoch != 0,
            Self::Checkpoint {
                lineage_identity,
                authority_epoch,
                checkpoint_identity,
                blob_closure_identity,
                ..
            } => {
                lineage_identity != [0; 32]
                    && authority_epoch != 0
                    && checkpoint_identity != [0; 32]
                    && blob_closure_identity != [0; 32]
            }
            Self::Wal {
                lineage_identity,
                authority_epoch,
                start_lsn,
                end_lsn_exclusive,
            } => {
                lineage_identity != [0; 32] && authority_epoch != 0 && start_lsn < end_lsn_exclusive
            }
            Self::Page {
                checkpoint_identity,
            }
            | Self::Layout {
                checkpoint_identity,
            } => checkpoint_identity != [0; 32],
            Self::Blob {
                blob_closure_identity,
            } => blob_closure_identity != [0; 32],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisasterRecoveryComponent {
    relative_path: PathBuf,
    evidence: DisasterRecoveryArtifactEvidence,
    semantics: DisasterRecoveryComponentSemantics,
}

impl DisasterRecoveryComponent {
    pub fn declare(
        relative_path: impl Into<PathBuf>,
        evidence: DisasterRecoveryArtifactEvidence,
        semantics: DisasterRecoveryComponentSemantics,
    ) -> Result<Self, DisasterRecoveryBundleDenial> {
        let relative_path = relative_path.into();
        if !valid_portable_relative_path(&relative_path) || !semantics.is_structurally_valid() {
            return Err(DisasterRecoveryBundleDenial::InvalidComponent);
        }
        Ok(Self {
            relative_path,
            evidence,
            semantics,
        })
    }

    pub const fn family(&self) -> DisasterRecoveryComponentFamily {
        self.semantics.family()
    }

    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    pub const fn evidence(&self) -> DisasterRecoveryArtifactEvidence {
        self.evidence
    }

    pub const fn semantics(&self) -> DisasterRecoveryComponentSemantics {
        self.semantics
    }

    pub const fn expected_digest(&self) -> [u8; 32] {
        self.evidence.expected_digest()
    }

    pub const fn byte_length(&self) -> u64 {
        self.evidence.byte_length()
    }
}

fn valid_portable_relative_path(path: &Path) -> bool {
    let Some(encoded) = path.to_str() else {
        return false;
    };
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && !encoded.contains(['\\', ':'])
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}
