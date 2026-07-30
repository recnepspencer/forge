//! Physical substrate evidence families.

mod blob;
mod foundational;
mod isolation;
mod physical_format;
mod physical_integrity;
mod recovery;

pub use blob::{
    LargeStorePressureEvidenceBundle, LargeStorePressureEvidenceDenial, LargeStoreShortcutAttempt,
};
pub use foundational::{
    certify_aspect_native_boundary_audit, AspectNativeBoundaryCertificationDenial,
    FoundationalPerformanceEvidenceDenial, PhysicalFoundationEvidenceBundle,
    PhysicalFoundationEvidenceBundleBuilder, PhysicalFoundationEvidenceDenial,
    PhysicalFoundationEvidenceEntry, PhysicalFoundationEvidenceIdentity,
};
pub use physical_format::{
    BinaryPhysicalFormatEvidence, BinaryPhysicalFormatEvidenceDenial,
    InMemoryPhysicalFormatModelEvidenceDenial, InMemoryPhysicalFormatModelEvidenceReport,
    InMemoryPhysicalFormatModelEvidenceRow, PhysicalComplexityEvidenceDenial,
    PhysicalComplexityEvidenceReport, PhysicalComplexityProofBundle,
    PhysicalExtentRecordFramingEvidenceDenial, PhysicalExtentRecordFramingEvidenceReport,
    PhysicalExtentRecordFramingEvidenceRow, PhysicalHeaderDecodeEvidenceDenial,
    PhysicalHeaderDecodeEvidenceReport, PhysicalHeaderDecodeEvidenceRow,
    PhysicalIdentityEvidenceDenial, PhysicalIdentityEvidenceReport, PhysicalIdentityEvidenceRow,
    PhysicalManifestDiscoveryEvidenceDenial, PhysicalManifestDiscoveryEvidenceReport,
    PhysicalManifestDiscoveryEvidenceRow, PhysicalPageRecordFramingEvidenceDenial,
    PhysicalPageRecordFramingEvidenceReport, PhysicalPageRecordFramingEvidenceRow,
};
pub use physical_integrity::{
    offline_observer_requires_physical_references, PhysicalOfflineVerifierEvidenceDenial,
    PhysicalOfflineVerifierEvidenceReport, PhysicalOfflineVerifierEvidenceRow,
};
