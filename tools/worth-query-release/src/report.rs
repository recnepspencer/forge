//! Descriptive machine-readable output for a completed release ceremony.

use serde::Serialize;
use sha2::{Digest, Sha256};
use worth_query_installation::facade::{
    WorthQueryPortableDomainPackageIdentity, WORTH_QUERY_PORTABLE_PACKAGE_MANIFEST_VERSION,
};
use worth_query_package_archive::facade::{
    WorthQueryUntrustedSignedPackageReleaseEnvelope, WORTH_QUERY_PACKAGE_ARCHIVE_PROTOCOL_VERSION,
    WORTH_QUERY_PACKAGE_ARCHIVE_RECORD_PROTOCOL_VERSION,
    WORTH_QUERY_PACKAGE_RELEASE_ENVELOPE_PROTOCOL_VERSION,
};

#[derive(Serialize)]
pub(crate) struct WorthQueryReleaseCeremonyReport {
    artifact_posture: &'static str,
    package_identity: String,
    archive_checksum: String,
    release_name: String,
    release_version: String,
    source_repository: String,
    source_revision: String,
    source_reference: String,
    signer_identity: String,
    signature_protocol_identity: String,
    signature_protocol_version: u32,
    envelope_protocol_version: u16,
    archive_protocol_version: u16,
    manifest_protocol_version: u16,
    record_protocol_version: u16,
    envelope_sha256: String,
}

impl WorthQueryReleaseCeremonyReport {
    pub(crate) fn derive(
        envelope: &WorthQueryUntrustedSignedPackageReleaseEnvelope,
        freshly_validated_identity: &WorthQueryPortableDomainPackageIdentity,
        envelope_bytes: &[u8],
    ) -> Self {
        let unsigned = envelope.envelope().unsigned();
        let release = unsigned.release_metadata();
        let provenance = unsigned.provenance();
        let signer = unsigned.signer();
        Self {
            artifact_posture: "untrusted-signed-envelope",
            package_identity: freshly_validated_identity.render_support_hex(),
            archive_checksum: encode_hex(unsigned.archive_checksum()),
            release_name: release.release_name().to_owned(),
            release_version: release.release_version().to_owned(),
            source_repository: provenance.source_repository().to_owned(),
            source_revision: provenance.source_revision().to_owned(),
            source_reference: provenance.source_reference().to_owned(),
            signer_identity: signer.signer_identity().to_owned(),
            signature_protocol_identity: signer.signature_protocol_identity().as_str().to_owned(),
            signature_protocol_version: signer.signature_protocol_version().get(),
            envelope_protocol_version: WORTH_QUERY_PACKAGE_RELEASE_ENVELOPE_PROTOCOL_VERSION,
            archive_protocol_version: WORTH_QUERY_PACKAGE_ARCHIVE_PROTOCOL_VERSION,
            manifest_protocol_version: WORTH_QUERY_PORTABLE_PACKAGE_MANIFEST_VERSION.get(),
            record_protocol_version: WORTH_QUERY_PACKAGE_ARCHIVE_RECORD_PROTOCOL_VERSION,
            envelope_sha256: encode_hex(&Sha256::digest(envelope_bytes)),
        }
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(DIGITS[usize::from(byte >> 4)]));
        encoded.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    encoded
}
