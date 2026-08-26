//! Descriptive signed-release envelope protocol.

mod checksum;
mod decoding;
mod descriptive_text;
mod descriptor;
mod encoding;
mod external_signature;
mod limits;
mod metadata;
mod protocol;
mod provenance;
mod requirements;
mod signature;
mod signed;
mod signing_payload;
mod unsigned;

pub use decoding::{
    decode_package_release_envelope, WorthQueryUntrustedSignedPackageReleaseEnvelope,
};
pub use descriptor::WorthQueryPackageReleaseEnvelopeDescriptor;
pub use encoding::{encode_package_release_envelope, prepare_package_release_envelope};
pub use external_signature::assemble_untrusted_package_release_envelope;
pub use limits::WorthQueryPackageEnvelopeLimits;
pub use metadata::{WorthQueryPackageBuildMetadata, WorthQueryPackageReleaseMetadata};
pub use protocol::WORTH_QUERY_PACKAGE_RELEASE_ENVELOPE_PROTOCOL_VERSION;
pub use provenance::WorthQueryPackageReleaseProvenance;
pub use requirements::WorthQueryPackageReleaseRequirements;
pub use signature::{
    WorthQueryPackageReleaseEnvelopeSignature, WorthQueryPackageReleaseSignerDescriptor,
};
pub use signed::WorthQuerySignedPackageReleaseEnvelope;
pub use signing_payload::{
    decode_package_release_signing_payload, WorthQueryUntrustedPackageReleaseSigningPayload,
};
pub use unsigned::WorthQueryUnsignedPackageReleaseEnvelope;

pub(crate) use checksum::WorthQueryPackageArchiveChecksum;
pub(crate) use descriptive_text::validate_descriptive_text;
pub(crate) use encoding::encode_signing_payload;
pub(crate) use protocol::{ENVELOPE_FIXED_HEADER_BYTES, ENVELOPE_MAGIC, SIGNATURE_LENGTH_BYTES};
