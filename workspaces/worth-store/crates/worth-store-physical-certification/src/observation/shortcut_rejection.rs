#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutRejectionObservationKind {
    PrivateMutationDenied,
    JsonAuthorityDenied,
    SameRunSelfComparisonDenied,
    WholeObjectHelperDenied,
    MissingChunkCountersDenied,
    LogOnlyEvidenceDenied,
    SyntheticSuccessRowDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortcutRejectionObservation {
    kind: ShortcutRejectionObservationKind,
}

impl ShortcutRejectionObservation {
    pub const fn private_mutation_denied() -> Self {
        Self {
            kind: ShortcutRejectionObservationKind::PrivateMutationDenied,
        }
    }

    pub const fn json_authority_denied() -> Self {
        Self {
            kind: ShortcutRejectionObservationKind::JsonAuthorityDenied,
        }
    }

    pub const fn same_run_self_comparison_denied() -> Self {
        Self {
            kind: ShortcutRejectionObservationKind::SameRunSelfComparisonDenied,
        }
    }

    pub const fn whole_object_helper_denied() -> Self {
        Self {
            kind: ShortcutRejectionObservationKind::WholeObjectHelperDenied,
        }
    }

    pub const fn missing_chunk_counters_denied() -> Self {
        Self {
            kind: ShortcutRejectionObservationKind::MissingChunkCountersDenied,
        }
    }

    pub const fn log_only_evidence_denied() -> Self {
        Self {
            kind: ShortcutRejectionObservationKind::LogOnlyEvidenceDenied,
        }
    }

    pub const fn synthetic_success_row_denied() -> Self {
        Self {
            kind: ShortcutRejectionObservationKind::SyntheticSuccessRowDenied,
        }
    }

    pub const fn kind(&self) -> ShortcutRejectionObservationKind {
        self.kind
    }
}
