//! Foundational substrate evidence re-exports.

pub use crate::evidence::foundational::handoff_gate_evidence::{
    certify_aspect_native_boundary_audit, AspectNativeBoundaryCertificationDenial,
};
pub use crate::evidence::foundational::performance_evidence_denial::FoundationalPerformanceEvidenceDenial;
pub use crate::evidence::physical_integrity::physical_foundation_evidence::{
    PhysicalFoundationEvidenceBundle, PhysicalFoundationEvidenceBundleBuilder,
    PhysicalFoundationEvidenceDenial, PhysicalFoundationEvidenceEntry,
    PhysicalFoundationEvidenceIdentity,
};
