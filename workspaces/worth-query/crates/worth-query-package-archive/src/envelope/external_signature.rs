//! Canonical re-entry after an external host signer produces signature bytes.

use crate::binary_output::BinaryOutput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

use super::limits::{require_complete_envelope_budget, require_signature_budget};
use super::{
    decode_package_release_envelope, WorthQueryPackageEnvelopeLimits,
    WorthQueryPackageReleaseEnvelopeSignature, WorthQueryUntrustedSignedPackageReleaseEnvelope,
};

/// Assemble canonical envelope bytes without treating signature presence as trust.
///
/// The signing payload may have crossed an external signing boundary. This
/// function owns the signature framing, then routes the complete bytes through
/// the ordinary bounded canonical decoder before returning them.
pub fn assemble_untrusted_package_release_envelope(
    signing_payload: &[u8],
    signature: WorthQueryPackageReleaseEnvelopeSignature,
    limits: WorthQueryPackageEnvelopeLimits,
) -> Result<WorthQueryUntrustedSignedPackageReleaseEnvelope, Denial> {
    require_signature_budget(signature.bytes().len(), limits)?;
    require_complete_envelope_budget(signing_payload.len(), signature.bytes().len(), limits)?;
    let signature_bytes = u32::try_from(signature.bytes().len())
        .map_err(|_| Denial::new(Kind::EnvelopeSignatureByteBudgetExceeded))?;
    let capacity = signing_payload
        .len()
        .checked_add(4)
        .and_then(|bytes| bytes.checked_add(signature.bytes().len()))
        .ok_or_else(|| Denial::new(Kind::EnvelopeByteBudgetExceeded))?;
    let mut output = BinaryOutput::with_capacity(capacity);
    output.raw_bytes(signing_payload);
    output.u32(signature_bytes);
    output.raw_bytes(signature.bytes());
    decode_package_release_envelope(&output.into_bytes(), limits)
}
