use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationOwner, PhysicalReference,
    PhysicalReferenceAdmissionWitness, SegmentGenerationCell,
};

use super::{mismatch_for_reference, mismatch_for_segment};
use crate::{GenerationCountedReferenceDenial, PhysicalReferenceGenerationMismatch};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationCountedPhysicalReference {
    AdmittedReference(PhysicalReference),
    Segment { owner: PhysicalGenerationOwner },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentGenerationPhysicalReference {
    reference: GenerationCountedPhysicalReference,
}

impl GenerationCountedPhysicalReference {
    pub const fn from_admitted_reference(admission: PhysicalReferenceAdmissionWitness) -> Self {
        Self::AdmittedReference(admission.reference())
    }

    pub const fn from_segment_cell(cell: SegmentGenerationCell) -> Self {
        Self::Segment {
            owner: cell.owner(),
        }
    }

    pub fn require_current_generation(
        self,
        observed_generation: PhysicalGeneration,
    ) -> Result<CurrentGenerationPhysicalReference, PhysicalReferenceGenerationMismatch> {
        match self {
            Self::AdmittedReference(reference) => {
                if reference.generation() == observed_generation {
                    Ok(CurrentGenerationPhysicalReference::from_validated_reference(self))
                } else {
                    Err(mismatch_for_reference(reference, observed_generation))
                }
            }
            Self::Segment { owner } => {
                if owner.generation() == observed_generation {
                    Ok(CurrentGenerationPhysicalReference::from_validated_reference(self))
                } else {
                    Err(mismatch_for_segment(
                        owner.generation(),
                        observed_generation,
                    ))
                }
            }
        }
    }

    pub const fn generation(self) -> PhysicalGeneration {
        match self {
            Self::AdmittedReference(reference) => reference.generation(),
            Self::Segment { owner } => owner.generation(),
        }
    }

    pub fn owner(self) -> PhysicalGenerationOwner {
        match self {
            Self::AdmittedReference(reference) => reference.generation_owner(),
            Self::Segment { owner } => owner,
        }
    }

    pub const fn reject_future_chunk_lifecycle_claim() -> GenerationCountedReferenceDenial {
        GenerationCountedReferenceDenial::FutureChunkLifecycleNotOwnedByS5
    }
}

impl CurrentGenerationPhysicalReference {
    const fn from_validated_reference(reference: GenerationCountedPhysicalReference) -> Self {
        Self { reference }
    }

    pub const fn generation(self) -> PhysicalGeneration {
        self.reference.generation()
    }

    pub fn owner(self) -> PhysicalGenerationOwner {
        self.reference.owner()
    }

    pub(crate) const fn generation_counted_reference(self) -> GenerationCountedPhysicalReference {
        self.reference
    }
}
