mod aggregate;
mod aggregate_accessors;
mod aggregate_construction;
mod certification;
mod certification_artifact_rows;
mod certification_contract_rows;
mod certification_evidence_rows;
mod certification_handoff_rows;
mod certification_matrix;
mod digests;
mod provenance;
mod raw_schema;
mod serialization;
mod upstream_artifacts;
mod validated_artifact;
mod validation;

pub use aggregate::S0EvidenceBundle;
pub use certification::{S0CertificationMatrixRow, S0CertificationStatus};
pub use provenance::{
    S0AcceptedEvidenceBundleWitness, S0ArtifactStalenessReport, S0EvidenceProvenance,
    S0RegenerationRequirement,
};
pub use validated_artifact::S0ValidatedEvidenceBundleArtifact;
pub use validation::{
    S0EvidenceBundleBuildRejection, S0EvidenceBundleParseRejection, S0StaleEvidenceRejection,
};
