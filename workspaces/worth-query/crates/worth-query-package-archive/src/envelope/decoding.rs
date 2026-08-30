use worth_query_installation::facade::WorthQueryPortableDomainPackageIdentity;

use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

use super::limits::require_signature_budget;
use super::signing_payload::{
    decode_package_release_signing_payload_prefix, require_envelope_budget,
};
use super::{
    WorthQueryPackageEnvelopeLimits, WorthQueryPackageReleaseEnvelopeSignature,
    WorthQuerySignedPackageReleaseEnvelope,
};

/// Structurally decoded signed envelope that has not passed host trust policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryUntrustedSignedPackageReleaseEnvelope {
    envelope: WorthQuerySignedPackageReleaseEnvelope,
}

impl WorthQueryUntrustedSignedPackageReleaseEnvelope {
    pub(crate) const fn new(envelope: WorthQuerySignedPackageReleaseEnvelope) -> Self {
        Self { envelope }
    }

    pub const fn envelope(&self) -> &WorthQuerySignedPackageReleaseEnvelope {
        &self.envelope
    }
    pub fn signing_payload(&self) -> &[u8] {
        self.envelope.signing_payload()
    }
    pub fn signature(&self) -> &[u8] {
        self.envelope.signature().bytes()
    }
    pub fn archive(&self) -> &[u8] {
        self.envelope.unsigned().archive()
    }
    pub const fn expected_package_identity(&self) -> &WorthQueryPortableDomainPackageIdentity {
        self.envelope.unsigned().expected_package_identity()
    }
    pub fn into_envelope(self) -> WorthQuerySignedPackageReleaseEnvelope {
        self.envelope
    }
}

pub fn decode_package_release_envelope(
    bytes: &[u8],
    limits: WorthQueryPackageEnvelopeLimits,
) -> Result<WorthQueryUntrustedSignedPackageReleaseEnvelope, Denial> {
    let limits = limits.narrowed();
    require_envelope_budget(bytes.len(), limits)?;
    let (payload, signing_payload_length) =
        decode_package_release_signing_payload_prefix(bytes, limits)?;
    let signature_source = bytes
        .get(signing_payload_length..)
        .ok_or_else(|| Denial::new(Kind::InvalidEnvelopeLength))?;
    let mut input = BinaryInput::new(signature_source);
    let signature_length = usize::try_from(input.u32()?)
        .map_err(|_| Denial::new(Kind::EnvelopeSignatureByteBudgetExceeded))?;
    require_signature_budget(signature_length, limits)?;
    let signature =
        WorthQueryPackageReleaseEnvelopeSignature::new(input.take(signature_length)?.to_vec())?;
    if !input.is_finished() {
        return Err(Denial::new(Kind::TrailingBytes));
    }
    let envelope = payload
        .into_unsigned()
        .attach_signature(signature, limits)?;
    Ok(WorthQueryUntrustedSignedPackageReleaseEnvelope::new(
        envelope,
    ))
}
