//! Buffer-pool substrate evidence re-exports.

pub use crate::evidence::cross_cutting::record_view_evidence::{
    RecordViewEvidenceDenial, RecordViewEvidenceReport, RecordViewEvidenceRow,
};
pub use crate::evidence::cross_cutting::speculative_work_evidence::{
    SpeculativeWorkEvidenceDenial, SpeculativeWorkEvidenceReport, SpeculativeWorkEvidenceRow,
};
pub use crate::evidence::durability::dirty_publication_evidence::{
    DirtyPublicationEvidenceDenial, DirtyPublicationEvidenceReport, DirtyPublicationEvidenceRow,
};
pub use crate::evidence::memory::allocation_envelope_evidence::{
    AllocationEnvelopeEvidenceDenial, AllocationEnvelopeEvidenceReport,
    AllocationEnvelopeEvidenceRow,
};
pub use crate::evidence::memory::eviction_protection_evidence::{
    EvictionProtectionEvidenceDenial, EvictionProtectionEvidenceReport,
    EvictionProtectionEvidenceRow,
};
pub use crate::evidence::memory::pin_lifecycle_evidence::{
    PinLifecycleEvidenceDenial, PinLifecycleEvidenceReport, PinLifecycleEvidenceRow,
};
pub use crate::evidence::memory::resident_frame_authority_evidence::{
    ResidentFrameAuthorityEvidenceDenial, ResidentFrameAuthorityEvidenceReport,
    ResidentFrameAuthorityEvidenceRow,
};
