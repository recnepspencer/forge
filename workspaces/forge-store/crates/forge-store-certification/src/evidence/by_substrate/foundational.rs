//! Foundational substrate evidence re-exports.

pub use crate::evidence::foundational::foundational_boundary_evidence::{
    AllocationEnvelopePerformanceReceipt, BufferPoolProvenanceAttachment,
    CompletedResidencyBoundaryReceipt, CopyMaterializationPerformanceReceipt,
    FoundationalBoundaryAuthorityResult, FoundationalBoundaryEvidenceDenial,
    FoundationalEvidenceProfile, FoundationalEvidenceRichness, MaterializationProfileReport,
    ResidentMemoryPerformanceReceipt, ZeroCopyLayoutPostureReport,
};
pub use crate::evidence::physical_integrity::physical_foundation_evidence::{
    PhysicalFoundationEvidenceBundle, PhysicalFoundationEvidenceBundleBuilder,
    PhysicalFoundationEvidenceDenial, PhysicalFoundationEvidenceEntry,
    PhysicalFoundationEvidenceIdentity,
};
pub use crate::evidence::foundational::handoff_gate_evidence::{
    certify_s0_handoff_gate_proof_evidence, S0HandoffGateCertificationDenial,
};
pub use crate::evidence::physical_substrate::entry_boundary_evidence::{
    S2EntryBoundaryEvidenceDenial, S2EntryBoundaryEvidenceReport, S2EntryBoundaryEvidenceRow,
    S2ForbiddenEntryAttempt,
};
