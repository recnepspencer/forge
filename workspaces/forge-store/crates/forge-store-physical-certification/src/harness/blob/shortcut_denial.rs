#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobHarnessShortcutAttempt {
    TinyBlob,
    WholeObjectHelper,
    MissingChunkCounters,
    LogsAsProof,
    SyntheticSuccessRow,
    PrivateHarnessStateMutation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobHarnessShortcutDenial {
    TinyBlobCannotSatisfyProfileEnvelope,
    WholeObjectHelperNotHarnessAuthority,
    MissingChunkCounters,
    LogsAreNotProof,
    SyntheticSuccessRowNotEvidence,
    PrivateMutationNotHarnessAuthority,
}

impl BlobHarnessShortcutAttempt {
    pub const fn tiny_blob() -> Self {
        Self::TinyBlob
    }

    pub const fn whole_object_helper() -> Self {
        Self::WholeObjectHelper
    }

    pub const fn missing_chunk_counters() -> Self {
        Self::MissingChunkCounters
    }

    pub const fn logs_as_proof() -> Self {
        Self::LogsAsProof
    }

    pub const fn synthetic_success_row() -> Self {
        Self::SyntheticSuccessRow
    }

    pub const fn private_harness_state_mutation() -> Self {
        Self::PrivateHarnessStateMutation
    }

    pub const fn deny_for_blob_harness(self) -> BlobHarnessShortcutDenial {
        match self {
            Self::TinyBlob => BlobHarnessShortcutDenial::TinyBlobCannotSatisfyProfileEnvelope,
            Self::WholeObjectHelper => {
                BlobHarnessShortcutDenial::WholeObjectHelperNotHarnessAuthority
            }
            Self::MissingChunkCounters => BlobHarnessShortcutDenial::MissingChunkCounters,
            Self::LogsAsProof => BlobHarnessShortcutDenial::LogsAreNotProof,
            Self::SyntheticSuccessRow => BlobHarnessShortcutDenial::SyntheticSuccessRowNotEvidence,
            Self::PrivateHarnessStateMutation => {
                BlobHarnessShortcutDenial::PrivateMutationNotHarnessAuthority
            }
        }
    }
}
