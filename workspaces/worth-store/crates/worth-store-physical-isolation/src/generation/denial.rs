use worth_store_physical_format::{PhysicalGeneration, PhysicalReference};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalReferenceGenerationMismatchKind {
    Page,
    Extent,
    Segment,
    RootPublication,
    FutureChunk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReferenceGenerationMismatch {
    kind: PhysicalReferenceGenerationMismatchKind,
    admitted_generation: PhysicalGeneration,
    observed_generation: PhysicalGeneration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationCountedReferenceDenial {
    ReferenceGenerationMismatch(PhysicalReferenceGenerationMismatch),
    WrongPhysicalReferenceKind,
    FutureChunkLifecycleNotOwnedByS5,
}

impl PhysicalReferenceGenerationMismatch {
    pub const fn new(
        kind: PhysicalReferenceGenerationMismatchKind,
        admitted_generation: PhysicalGeneration,
        observed_generation: PhysicalGeneration,
    ) -> Self {
        Self {
            kind,
            admitted_generation,
            observed_generation,
        }
    }

    pub const fn kind(self) -> PhysicalReferenceGenerationMismatchKind {
        self.kind
    }

    pub const fn admitted_generation(self) -> PhysicalGeneration {
        self.admitted_generation
    }

    pub const fn observed_generation(self) -> PhysicalGeneration {
        self.observed_generation
    }
}

pub(crate) fn mismatch_for_reference(
    reference: PhysicalReference,
    observed_generation: PhysicalGeneration,
) -> PhysicalReferenceGenerationMismatch {
    let kind = match reference.kind() {
        worth_store_physical_format::PhysicalReferenceKind::PageSlot => {
            PhysicalReferenceGenerationMismatchKind::Page
        }
        worth_store_physical_format::PhysicalReferenceKind::ExtentBacked
        | worth_store_physical_format::PhysicalReferenceKind::FreeSpaceReuse => {
            PhysicalReferenceGenerationMismatchKind::Extent
        }
        worth_store_physical_format::PhysicalReferenceKind::RootPublication => {
            PhysicalReferenceGenerationMismatchKind::RootPublication
        }
    };
    PhysicalReferenceGenerationMismatch::new(kind, reference.generation(), observed_generation)
}

pub(crate) const fn mismatch_for_segment(
    admitted_generation: PhysicalGeneration,
    observed_generation: PhysicalGeneration,
) -> PhysicalReferenceGenerationMismatch {
    PhysicalReferenceGenerationMismatch::new(
        PhysicalReferenceGenerationMismatchKind::Segment,
        admitted_generation,
        observed_generation,
    )
}

pub(crate) const fn mismatch_for_record_extent(
    admitted_generation: PhysicalGeneration,
    observed_generation: PhysicalGeneration,
) -> PhysicalReferenceGenerationMismatch {
    PhysicalReferenceGenerationMismatch::new(
        PhysicalReferenceGenerationMismatchKind::Extent,
        admitted_generation,
        observed_generation,
    )
}

pub(crate) const fn mismatch_for_page(
    admitted_generation: PhysicalGeneration,
    observed_generation: PhysicalGeneration,
) -> PhysicalReferenceGenerationMismatch {
    PhysicalReferenceGenerationMismatch::new(
        PhysicalReferenceGenerationMismatchKind::Page,
        admitted_generation,
        observed_generation,
    )
}
