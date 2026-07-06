use super::BlobPublicationCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPublicationDenial {
    RootCandidateRegistryMismatch {
        counters: BlobPublicationCounterSnapshot,
    },
    ReachabilityDigestMismatch {
        counters: BlobPublicationCounterSnapshot,
    },
    WalPublicationScopeRequired {
        counters: BlobPublicationCounterSnapshot,
    },
    WalReplayEvidenceRequired {
        counters: BlobPublicationCounterSnapshot,
    },
    WalReplayIdentityMismatch {
        counters: BlobPublicationCounterSnapshot,
    },
    CopiedPublicationRecordRejected {
        counters: BlobPublicationCounterSnapshot,
    },
    RootCandidateRejected {
        counters: BlobPublicationCounterSnapshot,
    },
    StagedReachabilityRejected {
        counters: BlobPublicationCounterSnapshot,
    },
    SemanticReferenceRejected {
        counters: BlobPublicationCounterSnapshot,
    },
    VisibilityRequiresPublishedGeneration {
        counters: BlobPublicationCounterSnapshot,
    },
}

pub const fn reject_root_candidate_as_blob_visibility<T>(_: &T) -> BlobPublicationDenial {
    BlobPublicationDenial::RootCandidateRejected {
        counters: BlobPublicationCounterSnapshot::start().with_denied_promotion(),
    }
}

pub const fn reject_staged_reachability_as_blob_visibility<T>(_: &T) -> BlobPublicationDenial {
    BlobPublicationDenial::StagedReachabilityRejected {
        counters: BlobPublicationCounterSnapshot::start().with_denied_promotion(),
    }
}

pub const fn reject_copied_publication_record_as_blob_visibility<T>(
    _: &T,
) -> BlobPublicationDenial {
    BlobPublicationDenial::CopiedPublicationRecordRejected {
        counters: BlobPublicationCounterSnapshot::start().with_denied_promotion(),
    }
}

pub const fn reject_semantic_reference_as_blob_visibility<T>(_: &T) -> BlobPublicationDenial {
    BlobPublicationDenial::SemanticReferenceRejected {
        counters: BlobPublicationCounterSnapshot::start().with_denied_promotion(),
    }
}
