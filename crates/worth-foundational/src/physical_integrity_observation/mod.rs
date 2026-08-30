//! Serializable physical-integrity facts shared across runtime boundaries.
//!
//! These values are descriptions only. They grant no media access, decoder
//! entry, recovery choice, quarantine mutation, or repair authority.

mod adapter_evidence;
mod artifact_family;
mod artifact_identity;
mod authority_class;
mod byte_range;
mod disagreement;
mod integrity_posture;
mod physical_generation;
mod quarantine_posture;
mod recovery_option;

pub use adapter_evidence::PhysicalAdapterEvidence;
pub use artifact_family::PhysicalArtifactFamily;
pub use artifact_identity::{PhysicalArtifactIdentity, PhysicalArtifactIdentityDenial};
pub use authority_class::PhysicalAuthorityClass;
pub use byte_range::{PhysicalByteRange, PhysicalByteRangeDenial};
pub use disagreement::PhysicalIntegrityDisagreement;
pub use integrity_posture::PhysicalIntegrityPosture;
pub use physical_generation::PhysicalArtifactGeneration;
pub use quarantine_posture::PhysicalQuarantinePosture;
pub use recovery_option::PhysicalRecoveryOption;
