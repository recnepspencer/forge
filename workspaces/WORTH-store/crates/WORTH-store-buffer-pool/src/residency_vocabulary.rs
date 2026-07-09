#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidencyVocabulary;

impl ResidencyVocabulary {
    pub const fn s2_phase_one() -> &'static [ResidencyAuthorityTerm] {
        &[
            ResidencyAuthorityTerm::ResidentMemory,
            ResidencyAuthorityTerm::PinnedPage,
            ResidencyAuthorityTerm::DirtyPage,
            ResidencyAuthorityTerm::CopiedBytes,
            ResidencyAuthorityTerm::MaterializedBytes,
            ResidencyAuthorityTerm::AllocationEnvelope,
            ResidencyAuthorityTerm::ReadinessHandoff,
            ResidencyAuthorityTerm::ResidentFrameTable,
            ResidencyAuthorityTerm::ResidentFrameGeneration,
            ResidencyAuthorityTerm::ResidentFrameHitMissCounters,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResidencyAuthorityTerm {
    ResidentMemory,
    PinnedPage,
    DirtyPage,
    CopiedBytes,
    MaterializedBytes,
    AllocationEnvelope,
    ReadinessHandoff,
    ResidentFrameTable,
    ResidentFrameGeneration,
    ResidentFrameHitMissCounters,
}
