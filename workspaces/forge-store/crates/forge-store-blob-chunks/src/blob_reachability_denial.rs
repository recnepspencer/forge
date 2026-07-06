use crate::BlobReachabilityCounterSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobReachabilityDenial {
    CopiedRefcountRowRejected {
        counters: BlobReachabilityCounterSnapshot,
    },
    EmptyReferenceProofRejected {
        counters: BlobReachabilityCounterSnapshot,
    },
    WrongBlobAuthority {
        counters: BlobReachabilityCounterSnapshot,
    },
    StaleGenerationEdge {
        counters: BlobReachabilityCounterSnapshot,
    },
    UnregisteredDedupeReceiptRejected {
        counters: BlobReachabilityCounterSnapshot,
    },
    DedupeReferenceMismatch {
        counters: BlobReachabilityCounterSnapshot,
    },
    InvalidProtectedHold {
        counters: BlobReachabilityCounterSnapshot,
    },
    ReclaimBlockedByReferenceEdge {
        counters: BlobReachabilityCounterSnapshot,
    },
    MissingReclaimReleaseEvidence {
        counters: BlobReachabilityCounterSnapshot,
    },
    BackendResidueRejected {
        counters: BlobReachabilityCounterSnapshot,
    },
    TerminalProjectionRejected {
        counters: BlobReachabilityCounterSnapshot,
    },
}

pub fn reject_copied_refcount_row_as_reachability(
    _row: &impl core::fmt::Debug,
) -> BlobReachabilityDenial {
    BlobReachabilityDenial::CopiedRefcountRowRejected {
        counters: BlobReachabilityCounterSnapshot::start().record_copied_row_denial(),
    }
}

pub fn reject_empty_reference_proof_as_reachability() -> BlobReachabilityDenial {
    BlobReachabilityDenial::EmptyReferenceProofRejected {
        counters: BlobReachabilityCounterSnapshot::start().record_empty_proof_denial(),
    }
}

pub fn reject_backend_residue_as_blob_reachability(
    _residue: &impl core::fmt::Debug,
) -> BlobReachabilityDenial {
    BlobReachabilityDenial::BackendResidueRejected {
        counters: BlobReachabilityCounterSnapshot::start().record_copied_row_denial(),
    }
}

pub fn reject_terminal_projection_as_blob_reachability(
    _projection: &impl core::fmt::Debug,
) -> BlobReachabilityDenial {
    BlobReachabilityDenial::TerminalProjectionRejected {
        counters: BlobReachabilityCounterSnapshot::start().record_copied_row_denial(),
    }
}
