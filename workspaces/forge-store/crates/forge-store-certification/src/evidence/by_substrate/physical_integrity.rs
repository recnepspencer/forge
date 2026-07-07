//! Physical-integrity substrate evidence re-exports.

pub use crate::offline_verifier_evidence::{
    offline_observer_requires_physical_references, PhysicalOfflineVerifierEvidenceDenial,
    PhysicalOfflineVerifierEvidenceReport, PhysicalOfflineVerifierEvidenceRow,
};
pub use crate::protected_integrity_view_evidence::{
    ProtectedIntegrityViewEvidence, ProtectedIntegrityViewEvidenceDenial,
};
