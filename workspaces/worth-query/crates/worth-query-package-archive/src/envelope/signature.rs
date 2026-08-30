use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};

use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

use super::validate_descriptive_text;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageReleaseSignerDescriptor {
    signer_identity: String,
    signature_protocol_identity: BoundaryProtocolIdentity,
    signature_protocol_version: BoundaryProtocolVersion,
}

impl WorthQueryPackageReleaseSignerDescriptor {
    pub fn new(
        signer_identity: impl Into<String>,
        signature_protocol_identity: BoundaryProtocolIdentity,
        signature_protocol_version: BoundaryProtocolVersion,
    ) -> Result<Self, Denial> {
        let signer_identity = signer_identity.into();
        validate_descriptive_text(&signer_identity)?;
        Ok(Self {
            signer_identity,
            signature_protocol_identity,
            signature_protocol_version,
        })
    }

    pub fn signer_identity(&self) -> &str {
        &self.signer_identity
    }
    pub const fn signature_protocol_identity(&self) -> &BoundaryProtocolIdentity {
        &self.signature_protocol_identity
    }
    pub const fn signature_protocol_version(&self) -> BoundaryProtocolVersion {
        self.signature_protocol_version
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageReleaseEnvelopeSignature(Vec<u8>);

impl WorthQueryPackageReleaseEnvelopeSignature {
    pub fn new(bytes: Vec<u8>) -> Result<Self, Denial> {
        if bytes.is_empty() {
            return Err(Denial::new(Kind::EmptyEnvelopeSignature));
        }
        Ok(Self(bytes))
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}
