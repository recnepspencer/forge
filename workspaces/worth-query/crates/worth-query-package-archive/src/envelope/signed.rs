use super::{WorthQueryPackageReleaseEnvelopeSignature, WorthQueryUnsignedPackageReleaseEnvelope};

/// Descriptive signed envelope. Signature presence is not signer trust.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySignedPackageReleaseEnvelope {
    unsigned: WorthQueryUnsignedPackageReleaseEnvelope,
    signature: WorthQueryPackageReleaseEnvelopeSignature,
}

impl WorthQuerySignedPackageReleaseEnvelope {
    pub(crate) const fn new(
        unsigned: WorthQueryUnsignedPackageReleaseEnvelope,
        signature: WorthQueryPackageReleaseEnvelopeSignature,
    ) -> Self {
        Self {
            unsigned,
            signature,
        }
    }

    pub const fn unsigned(&self) -> &WorthQueryUnsignedPackageReleaseEnvelope {
        &self.unsigned
    }
    pub const fn signature(&self) -> &WorthQueryPackageReleaseEnvelopeSignature {
        &self.signature
    }
    pub fn signing_payload(&self) -> &[u8] {
        self.unsigned.signing_payload()
    }
}
