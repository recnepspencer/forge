use std::path::Path;

use sha2::{Digest, Sha256};

use super::{
    ExternalToolIdentity, PinnedTlcToolchain, ProtocolCheckInvocation, ProtocolCheckVerdict,
};
use crate::ProtocolFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolCheckArtifactIdentity {
    model_sha256: [u8; 32],
    configuration_sha256: [u8; 32],
    tool_sha256: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedProtocolCheck {
    invocation: ProtocolCheckInvocation,
    artifact_identity: ProtocolCheckArtifactIdentity,
    external_tool_identity: ExternalToolIdentity,
    verdict: ProtocolCheckVerdict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolArtifactIdentityInspectionDenial {
    ArtifactRead(String),
    InvalidPinnedToolDigest,
}

impl ProtocolCheckArtifactIdentity {
    pub(super) const fn observed(
        model_sha256: [u8; 32],
        configuration_sha256: [u8; 32],
        tool_sha256: [u8; 32],
    ) -> Self {
        Self {
            model_sha256,
            configuration_sha256,
            tool_sha256,
        }
    }

    pub fn declared_for(
        invocation: &ProtocolCheckInvocation,
    ) -> Result<Self, ProtocolArtifactIdentityInspectionDenial> {
        Ok(Self {
            model_sha256: file_digest(invocation.model_path())?,
            configuration_sha256: file_digest(invocation.configuration_path())?,
            tool_sha256: decode_pinned_tool_digest()?,
        })
    }

    pub const fn model_sha256(&self) -> &[u8; 32] {
        &self.model_sha256
    }

    pub const fn configuration_sha256(&self) -> &[u8; 32] {
        &self.configuration_sha256
    }

    pub const fn tool_sha256(&self) -> &[u8; 32] {
        &self.tool_sha256
    }
}

fn file_digest(path: &Path) -> Result<[u8; 32], ProtocolArtifactIdentityInspectionDenial> {
    let bytes = std::fs::read(path).map_err(|error| {
        ProtocolArtifactIdentityInspectionDenial::ArtifactRead(error.to_string())
    })?;
    Ok(Sha256::digest(bytes).into())
}

fn decode_pinned_tool_digest() -> Result<[u8; 32], ProtocolArtifactIdentityInspectionDenial> {
    let bytes = PinnedTlcToolchain::SHA256.as_bytes();
    if bytes.len() != 64 {
        return Err(ProtocolArtifactIdentityInspectionDenial::InvalidPinnedToolDigest);
    }
    let mut digest = [0_u8; 32];
    for (index, byte) in digest.iter_mut().enumerate() {
        let high = decode_hex_digit(bytes[index * 2])?;
        let low = decode_hex_digit(bytes[index * 2 + 1])?;
        *byte = (high << 4) | low;
    }
    Ok(digest)
}

fn decode_hex_digit(byte: u8) -> Result<u8, ProtocolArtifactIdentityInspectionDenial> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(ProtocolArtifactIdentityInspectionDenial::InvalidPinnedToolDigest),
    }
}

impl ExecutedProtocolCheck {
    pub(super) const fn observed(
        invocation: ProtocolCheckInvocation,
        artifact_identity: ProtocolCheckArtifactIdentity,
        external_tool_identity: ExternalToolIdentity,
        verdict: ProtocolCheckVerdict,
    ) -> Self {
        Self {
            invocation,
            artifact_identity,
            external_tool_identity,
            verdict,
        }
    }

    pub const fn protocol(&self) -> ProtocolFamily {
        self.invocation.protocol()
    }

    pub const fn invocation(&self) -> &ProtocolCheckInvocation {
        &self.invocation
    }

    pub const fn artifact_identity(&self) -> &ProtocolCheckArtifactIdentity {
        &self.artifact_identity
    }

    pub const fn external_tool_identity(&self) -> &ExternalToolIdentity {
        &self.external_tool_identity
    }

    pub const fn verdict(&self) -> &ProtocolCheckVerdict {
        &self.verdict
    }

    pub fn into_parts(
        self,
    ) -> (
        ProtocolCheckInvocation,
        ProtocolCheckArtifactIdentity,
        ExternalToolIdentity,
        ProtocolCheckVerdict,
    ) {
        (
            self.invocation,
            self.artifact_identity,
            self.external_tool_identity,
            self.verdict,
        )
    }

    pub fn into_verdict(self) -> ProtocolCheckVerdict {
        self.verdict
    }
}
