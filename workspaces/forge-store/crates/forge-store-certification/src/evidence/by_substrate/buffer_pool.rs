//! Buffer-pool substrate evidence re-exports.

pub use crate::allocation_envelope_evidence::{
    AllocationEnvelopeEvidenceDenial, AllocationEnvelopeEvidenceReport, AllocationEnvelopeEvidenceRow,
};
pub use crate::dirty_publication_evidence::{
    DirtyPublicationEvidenceDenial, DirtyPublicationEvidenceReport, DirtyPublicationEvidenceRow,
};
pub use crate::eviction_protection_evidence::{
    EvictionProtectionEvidenceDenial, EvictionProtectionEvidenceReport, EvictionProtectionEvidenceRow,
};
pub use crate::pin_lifecycle_evidence::{
    PinLifecycleEvidenceDenial, PinLifecycleEvidenceReport, PinLifecycleEvidenceRow,
};
pub use crate::record_view_evidence::{
    RecordViewEvidenceDenial, RecordViewEvidenceReport, RecordViewEvidenceRow,
};
pub use crate::resident_frame_authority_evidence::{
    ResidentFrameAuthorityEvidenceDenial, ResidentFrameAuthorityEvidenceReport,
    ResidentFrameAuthorityEvidenceRow,
};
pub use crate::speculative_work_evidence::{
    SpeculativeWorkEvidenceDenial, SpeculativeWorkEvidenceReport, SpeculativeWorkEvidenceRow,
};