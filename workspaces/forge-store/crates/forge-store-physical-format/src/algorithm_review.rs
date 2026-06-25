use crate::{PhysicalLocalityClass, PhysicalOperationKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalAlgorithmReviewEvidence {
    operation: PhysicalOperationKind,
    locality: PhysicalLocalityClass,
    conclusion: PhysicalAlgorithmReviewConclusion,
}

impl PhysicalAlgorithmReviewEvidence {
    pub const fn bounded_by_admitted_reference() -> Self {
        Self::new(
            PhysicalOperationKind::LocateByReference,
            PhysicalLocalityClass::PageLocal,
            PhysicalAlgorithmReviewConclusion::NoUnrelatedMetadataTraversal,
        )
    }

    pub const fn constant_header_decode() -> Self {
        Self::new(
            PhysicalOperationKind::HeaderDecode,
            PhysicalLocalityClass::Constant,
            PhysicalAlgorithmReviewConclusion::BoundedByFixedHeaderFields,
        )
    }

    pub const fn constant_reference_validation() -> Self {
        Self::new(
            PhysicalOperationKind::PhysicalReferenceValidation,
            PhysicalLocalityClass::Constant,
            PhysicalAlgorithmReviewConclusion::BoundedByReferenceFields,
        )
    }

    pub const fn manifest_lookup() -> Self {
        Self::new(
            PhysicalOperationKind::ManifestLookup,
            PhysicalLocalityClass::SegmentLocal,
            PhysicalAlgorithmReviewConclusion::BoundedByManifestIndex,
        )
    }

    pub const fn root_manifest_open() -> Self {
        Self::new(
            PhysicalOperationKind::RootManifestOpen,
            PhysicalLocalityClass::RootManifest,
            PhysicalAlgorithmReviewConclusion::DeclaredRootManifestTraversal,
        )
    }

    pub const fn bounded_append_placement() -> Self {
        Self::new(
            PhysicalOperationKind::AppendRecordPlacement,
            PhysicalLocalityClass::FreeSpaceClass,
            PhysicalAlgorithmReviewConclusion::BoundedByFreeSpacePolicy,
        )
    }

    pub const fn manifest_traversal() -> Self {
        Self::new(
            PhysicalOperationKind::ManifestTraversal,
            PhysicalLocalityClass::ManifestDeclaredTraversal,
            PhysicalAlgorithmReviewConclusion::ExplicitTraversalOnly,
        )
    }

    pub const fn offline_verifier_walk() -> Self {
        Self::new(
            PhysicalOperationKind::OfflineVerifierWalk,
            PhysicalLocalityClass::ManifestDeclaredTraversal,
            PhysicalAlgorithmReviewConclusion::ExplicitTraversalOnly,
        )
    }

    pub const fn operation(self) -> PhysicalOperationKind {
        self.operation
    }

    pub const fn locality(self) -> PhysicalLocalityClass {
        self.locality
    }

    pub const fn conclusion(self) -> PhysicalAlgorithmReviewConclusion {
        self.conclusion
    }

    const fn new(
        operation: PhysicalOperationKind,
        locality: PhysicalLocalityClass,
        conclusion: PhysicalAlgorithmReviewConclusion,
    ) -> Self {
        Self {
            operation,
            locality,
            conclusion,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalAlgorithmReviewConclusion {
    BoundedByFixedHeaderFields,
    BoundedByReferenceFields,
    NoUnrelatedMetadataTraversal,
    BoundedByManifestIndex,
    DeclaredRootManifestTraversal,
    BoundedByFreeSpacePolicy,
    ExplicitTraversalOnly,
}
