use crate::{
    PhysicalGenerationOwner, PhysicalReference, PhysicalReferenceValidationCounterSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReferenceAdmissionWitness {
    reference: PhysicalReference,
}

impl PhysicalReferenceAdmissionWitness {
    pub(crate) const fn new(reference: PhysicalReference) -> Self {
        Self { reference }
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub fn owner(self) -> PhysicalGenerationOwner {
        self.reference.generation_owner()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReferenceValidationWitness {
    reference: PhysicalReference,
    counters: PhysicalReferenceValidationCounterSnapshot,
}

impl PhysicalReferenceValidationWitness {
    pub(crate) const fn new(
        reference: PhysicalReference,
        counters: PhysicalReferenceValidationCounterSnapshot,
    ) -> Self {
        Self {
            reference,
            counters,
        }
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub const fn counters(self) -> PhysicalReferenceValidationCounterSnapshot {
        self.counters
    }

    pub fn owner(self) -> PhysicalGenerationOwner {
        self.reference.generation_owner()
    }

    pub const fn admission(self) -> PhysicalReferenceAdmissionWitness {
        PhysicalReferenceAdmissionWitness::new(self.reference)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RootPublicationValidationWitness {
    validation: PhysicalReferenceValidationWitness,
}

impl RootPublicationValidationWitness {
    pub(crate) const fn new(validation: PhysicalReferenceValidationWitness) -> Self {
        Self { validation }
    }

    pub const fn reference(self) -> PhysicalReference {
        self.validation.reference()
    }

    pub const fn counters(self) -> PhysicalReferenceValidationCounterSnapshot {
        self.validation.counters()
    }

    pub fn owner(self) -> PhysicalGenerationOwner {
        self.validation.owner()
    }

    pub const fn as_physical_reference_validation(self) -> PhysicalReferenceValidationWitness {
        self.validation
    }

    pub const fn admission(self) -> PhysicalReferenceAdmissionWitness {
        self.validation.admission()
    }
}
