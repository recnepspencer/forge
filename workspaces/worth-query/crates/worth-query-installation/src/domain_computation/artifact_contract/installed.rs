use crate::generation::WorthQueryInstallationGeneration;
use crate::package::WorthQueryPortableDomainPackageIdentity;

use super::WorthQueryPortableArtifactContract;

/// Opaque proof that one exact artifact contract belongs to an installed
/// package, runtime, and generation. This is contract authority, not a payload
/// handle or occurrence authority.
///
/// Serialized fields and copied semantic identities cannot construct it:
///
/// ```compile_fail
/// use worth_query_installation::facade::{
///     WorthQueryInstallationGeneration,
///     WorthQueryInstalledArtifactContractAuthority,
/// };
///
/// let _forged = WorthQueryInstalledArtifactContractAuthority {
///     runtime_ordinal: 7,
///     generation: WorthQueryInstallationGeneration::initial(),
///     owner: "worth.routing".into(),
///     package_identity: panic!("copied package identity"),
///     admission_identity: "copied-admission-digest".into(),
///     package_authority_nonce: [0; 32],
///     contract: panic!("reconstructed contract"),
/// };
/// ```
///
/// Descriptive transformation evidence is not installation authority:
///
/// ```compile_fail
/// use worth_query_installation::facade::{
///     WorthQueryImmutableSourceOccurrenceContract,
///     WorthQueryInstalledArtifactContractAuthority,
///     WorthQuerySourceOutputCorrespondence,
///     WorthQueryTransformationDisposition,
///     WorthQueryTransformationErrorPosture,
///     WorthQueryTransformationEvidenceContract,
///     WorthQueryTransformationIdentity,
///     WorthQueryTransformationLossPosture,
///     WorthQueryTransformationOutcomeContract,
/// };
///
/// fn requires_installation(_: WorthQueryInstalledArtifactContractAuthority) {}
/// let copied_evidence = WorthQueryTransformationEvidenceContract::declared(
///     WorthQueryImmutableSourceOccurrenceContract::new("copied.source"),
///     WorthQueryTransformationIdentity::new("copied.transformation", 1),
///     WorthQueryTransformationOutcomeContract::new(
///         WorthQuerySourceOutputCorrespondence::OneToOne,
///         WorthQueryTransformationDisposition::Preserved,
///         WorthQueryTransformationErrorPosture::Exact,
///         WorthQueryTransformationLossPosture::Lossless,
///     ),
/// );
/// requires_installation(copied_evidence);
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledArtifactContractAuthority {
    pub(crate) runtime_ordinal: u64,
    pub(crate) generation: WorthQueryInstallationGeneration,
    pub(crate) owner: String,
    pub(crate) package_identity: WorthQueryPortableDomainPackageIdentity,
    pub(crate) admission_identity: String,
    pub(crate) package_authority_nonce: [u8; 32],
    pub(crate) contract: WorthQueryPortableArtifactContract,
}

impl WorthQueryInstalledArtifactContractAuthority {
    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn contract(&self) -> &WorthQueryPortableArtifactContract {
        &self.contract
    }

    pub fn package_identity(&self) -> &WorthQueryPortableDomainPackageIdentity {
        &self.package_identity
    }

    pub fn admission_identity(&self) -> &str {
        &self.admission_identity
    }
}
