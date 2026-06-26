use forge_store_aspect_native::{StoreAspectBoundaryFact, StorePhysicalBoundaryWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectNativeVocabularyFamily {
    AspectValues,
    ValidatedValues,
    AuthoritativeState,
    AuthoritativePatch,
    Locators,
    Receipts,
    Diagnostics,
    PerformanceEvidence,
    StorePhysicalWitness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectNativeVocabularyPosture {
    FoundationalSharedVocabulary,
    StoreOwnedPhysicalWitness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreAspectNativeVocabularyReadiness {
    boundary_fact: StoreAspectBoundaryFact,
    physical_witness: StorePhysicalBoundaryWitness,
    adopted_families: Vec<(AspectNativeVocabularyFamily, AspectNativeVocabularyPosture)>,
}

impl StoreAspectNativeVocabularyReadiness {
    pub fn from_boundary_fact(
        boundary_fact: StoreAspectBoundaryFact,
        physical_witness: StorePhysicalBoundaryWitness,
    ) -> Self {
        Self {
            boundary_fact,
            physical_witness,
            adopted_families: aspect_native_vocabulary_families(),
        }
    }

    pub const fn boundary_fact(&self) -> &StoreAspectBoundaryFact {
        &self.boundary_fact
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }

    pub fn adopted_families(
        &self,
    ) -> &[(AspectNativeVocabularyFamily, AspectNativeVocabularyPosture)] {
        &self.adopted_families
    }
}

fn aspect_native_vocabulary_families(
) -> Vec<(AspectNativeVocabularyFamily, AspectNativeVocabularyPosture)> {
    use AspectNativeVocabularyFamily as Family;
    use AspectNativeVocabularyPosture as Posture;

    vec![
        (Family::AspectValues, Posture::FoundationalSharedVocabulary),
        (
            Family::ValidatedValues,
            Posture::FoundationalSharedVocabulary,
        ),
        (
            Family::AuthoritativeState,
            Posture::FoundationalSharedVocabulary,
        ),
        (
            Family::AuthoritativePatch,
            Posture::FoundationalSharedVocabulary,
        ),
        (Family::Locators, Posture::FoundationalSharedVocabulary),
        (Family::Receipts, Posture::FoundationalSharedVocabulary),
        (Family::Diagnostics, Posture::FoundationalSharedVocabulary),
        (
            Family::PerformanceEvidence,
            Posture::FoundationalSharedVocabulary,
        ),
        (
            Family::StorePhysicalWitness,
            Posture::StoreOwnedPhysicalWitness,
        ),
    ]
}
