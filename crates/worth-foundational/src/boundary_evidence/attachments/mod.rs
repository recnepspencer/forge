mod bundle;
mod continuity;
mod definitions;
mod descriptive;
mod materialization;
mod readmission;
mod target;

pub use bundle::{
    FoundationalBoundaryEvidenceAttachmentBundle, FoundationalDiagnosticBundleAttachmentBundle,
};
pub use continuity::{
    FoundationalBoundaryEvidenceLocatorContinuityAttachment,
    FoundationalBoundaryEvidenceObjectContinuityAttachment,
};
pub use definitions::{
    foundational_boundary_evidence_attachment_target_kind_definitions,
    foundational_boundary_evidence_continuity_attachment_scope_definitions,
    foundational_boundary_evidence_materialization_profile_definitions,
    FoundationalBoundaryEvidenceAttachmentTargetKind,
    FoundationalBoundaryEvidenceContinuityAttachmentScope,
    FoundationalBoundaryEvidenceMaterializationProfile,
};
pub use descriptive::{
    FoundationalBoundaryEvidenceDiagnosticAttachment, FoundationalBoundaryEvidenceSupportAttachment,
};
pub use materialization::{
    derive_boundary_evidence_attachment_bundle_digest,
    prepare_boundary_evidence_attachment_bundle_for_canonical_basis,
    FoundationalBoundaryEvidenceAttachmentDigestDerivationDenial,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
};
pub use readmission::{
    admit_current_basis_boundary_evidence_attachment_bundle,
    admit_support_basis_boundary_evidence_attachment_bundle,
    bridge_current_basis_boundary_evidence_attachment_bundle_trust_boundary,
    bridge_support_basis_boundary_evidence_attachment_bundle_trust_boundary,
    foundational_boundary_evidence_attachment_readmission_authority,
    foundational_boundary_evidence_support_readmission_authority,
    readmit_current_basis_boundary_evidence_attachment_bundle_after_boundary,
    readmit_support_basis_boundary_evidence_attachment_bundle_after_boundary,
    BoundaryBridgedCurrentBasisBoundaryEvidenceAttachmentBundle,
    BoundaryBridgedSupportBasisBoundaryEvidenceAttachmentBundle,
    CurrentBasisBoundaryEvidenceAttachmentBundle,
    FoundationalBoundaryEvidenceAttachmentReadmissionAuthority,
    FoundationalBoundaryEvidenceSupportReadmissionAuthority,
    FoundationalBoundaryEvidenceSupportReadmissionDenial,
    SupportBasisBoundaryEvidenceAttachmentBundle,
};
pub use target::FoundationalBoundaryEvidenceAttachmentTarget;

pub(crate) use descriptive::{
    canonical_fragment_for_provenance_attachment, canonical_fragment_for_receipt_attachment,
};
