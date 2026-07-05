use crate::BlobGenerationRegistryCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobGenerationRegistryDenial {
    RootPublicationLifecycleRootMismatch {
        counters: BlobGenerationRegistryCounterSnapshot,
    },
    RootPublicationLifecycleDigestMismatch {
        counters: BlobGenerationRegistryCounterSnapshot,
    },
    ClassificationLifecycleBindingMismatch {
        counters: BlobGenerationRegistryCounterSnapshot,
    },
    BlobGenerationAlreadyBoundDifferently {
        counters: BlobGenerationRegistryCounterSnapshot,
    },
    BlobGenerationNotPublished {
        counters: BlobGenerationRegistryCounterSnapshot,
    },
    DigestEqualityRejected,
    ChunkTreeEqualityRejected,
    CopiedLifecycleReceiptRejected,
    SemanticReferenceIdRejected,
    RawGenerationNumberRejected,
    TerminalProjectionRowRejected,
    PhysicalGenerationRejected,
    DerivedRebuildAuthorityRequired {
        counters: BlobGenerationRegistryCounterSnapshot,
    },
    AuthoritativeBlobRequiresAuthoritativeRepair {
        counters: BlobGenerationRegistryCounterSnapshot,
    },
}

pub const fn reject_digest_equality_as_blob_identity<T>(_: &T) -> BlobGenerationRegistryDenial {
    BlobGenerationRegistryDenial::DigestEqualityRejected
}

pub const fn reject_chunk_tree_equality_as_blob_identity<T>(_: &T) -> BlobGenerationRegistryDenial {
    BlobGenerationRegistryDenial::ChunkTreeEqualityRejected
}

pub const fn reject_copied_lifecycle_receipt_as_blob_identity<T>(
    _: &T,
) -> BlobGenerationRegistryDenial {
    BlobGenerationRegistryDenial::CopiedLifecycleReceiptRejected
}

pub const fn reject_semantic_reference_id_as_blob_identity<T>(
    _: &T,
) -> BlobGenerationRegistryDenial {
    BlobGenerationRegistryDenial::SemanticReferenceIdRejected
}

pub const fn reject_raw_generation_number_as_blob_identity(_: u64) -> BlobGenerationRegistryDenial {
    BlobGenerationRegistryDenial::RawGenerationNumberRejected
}

pub const fn reject_terminal_projection_row_as_blob_identity<T>(
    _: &T,
) -> BlobGenerationRegistryDenial {
    BlobGenerationRegistryDenial::TerminalProjectionRowRejected
}

pub const fn reject_physical_generation_as_blob_generation<T>(
    _: &T,
) -> BlobGenerationRegistryDenial {
    BlobGenerationRegistryDenial::PhysicalGenerationRejected
}
