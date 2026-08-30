use worth_query_installation::facade::WorthQueryPortableDomainPackageIdentity;

use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

use super::limits::{require_complete_envelope_budget, require_signature_budget};
use super::{
    WorthQueryPackageArchiveChecksum, WorthQueryPackageBuildMetadata,
    WorthQueryPackageEnvelopeLimits, WorthQueryPackageReleaseEnvelopeDescriptor,
    WorthQueryPackageReleaseEnvelopeSignature, WorthQueryPackageReleaseMetadata,
    WorthQueryPackageReleaseProvenance, WorthQueryPackageReleaseRequirements,
    WorthQueryPackageReleaseSignerDescriptor, WorthQuerySignedPackageReleaseEnvelope,
};

/// Complete canonical release-envelope body before a host attaches a signature.
///
/// This value is descriptive and carries no Query validation, host trust, or
/// activation authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryUnsignedPackageReleaseEnvelope {
    archive: Vec<u8>,
    expected_package_identity: WorthQueryPortableDomainPackageIdentity,
    archive_checksum: WorthQueryPackageArchiveChecksum,
    descriptor: WorthQueryPackageReleaseEnvelopeDescriptor,
    requirements: WorthQueryPackageReleaseRequirements,
    signing_payload: Vec<u8>,
}

impl WorthQueryUnsignedPackageReleaseEnvelope {
    pub(crate) fn new(
        archive: Vec<u8>,
        expected_package_identity: WorthQueryPortableDomainPackageIdentity,
        archive_checksum: WorthQueryPackageArchiveChecksum,
        descriptor: WorthQueryPackageReleaseEnvelopeDescriptor,
        requirements: WorthQueryPackageReleaseRequirements,
        signing_payload: Vec<u8>,
    ) -> Self {
        Self {
            archive,
            expected_package_identity,
            archive_checksum,
            descriptor,
            requirements,
            signing_payload,
        }
    }

    pub fn archive(&self) -> &[u8] {
        &self.archive
    }
    pub const fn expected_package_identity(&self) -> &WorthQueryPortableDomainPackageIdentity {
        &self.expected_package_identity
    }
    pub const fn archive_checksum(&self) -> &[u8; 32] {
        self.archive_checksum.bytes()
    }
    pub const fn build_metadata(&self) -> &WorthQueryPackageBuildMetadata {
        self.descriptor.build_metadata()
    }
    pub const fn release_metadata(&self) -> &WorthQueryPackageReleaseMetadata {
        self.descriptor.release_metadata()
    }
    pub const fn provenance(&self) -> &WorthQueryPackageReleaseProvenance {
        self.descriptor.provenance()
    }
    pub const fn requirements(&self) -> &WorthQueryPackageReleaseRequirements {
        &self.requirements
    }
    pub const fn signer(&self) -> &WorthQueryPackageReleaseSignerDescriptor {
        self.descriptor.signer()
    }
    pub const fn descriptor(&self) -> &WorthQueryPackageReleaseEnvelopeDescriptor {
        &self.descriptor
    }
    pub fn signing_payload(&self) -> &[u8] {
        &self.signing_payload
    }

    /// Prove that a host-expected opaque signature shape fits this envelope.
    ///
    /// This is a byte-capacity check only. It grants no signer trust and does
    /// not verify a signature protocol.
    pub fn require_external_signature_capacity(
        &self,
        signature_bytes: u32,
        limits: WorthQueryPackageEnvelopeLimits,
    ) -> Result<(), Denial> {
        if signature_bytes == 0 {
            return Err(Denial::new(Kind::EmptyEnvelopeSignature));
        }
        let signature_bytes = usize::try_from(signature_bytes)
            .map_err(|_| Denial::new(Kind::EnvelopeSignatureByteBudgetExceeded))?;
        require_signature_budget(signature_bytes, limits)?;
        require_complete_envelope_budget(self.signing_payload.len(), signature_bytes, limits)
    }

    pub fn attach_signature(
        self,
        signature: WorthQueryPackageReleaseEnvelopeSignature,
        limits: WorthQueryPackageEnvelopeLimits,
    ) -> Result<WorthQuerySignedPackageReleaseEnvelope, Denial> {
        let signature_bytes = u32::try_from(signature.bytes().len())
            .map_err(|_| Denial::new(Kind::EnvelopeSignatureByteBudgetExceeded))?;
        self.require_external_signature_capacity(signature_bytes, limits)?;
        Ok(WorthQuerySignedPackageReleaseEnvelope::new(self, signature))
    }
}
