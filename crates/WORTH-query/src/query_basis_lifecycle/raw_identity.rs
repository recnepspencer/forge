use worth_runtime_bridge::facade::BridgeIdentityEvidence;

use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceIdentityEncoder, WorthQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RawBasisIdentity {
    Query(WorthQueryEvidenceIdentity),
    Bridge(BridgeIdentityEvidence),
}

impl RawBasisIdentity {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Query(identity) => identity.terminal_projection_for_reporting(),
            Self::Bridge(identity) => identity.terminal_projection_for_reporting(),
        }
    }

    pub(super) fn encode(
        &self,
        encoder: WorthQueryEvidenceIdentityEncoder,
        tag: WorthQueryEvidenceTag,
    ) -> WorthQueryEvidenceIdentityEncoder {
        match self {
            Self::Query(identity) => encoder.field_evidence_identity(tag, identity),
            Self::Bridge(identity) => {
                encoder.field_bridge_retained_evidence_identity(tag, identity)
            }
        }
    }
}

impl From<WorthQueryEvidenceIdentity> for RawBasisIdentity {
    fn from(identity: WorthQueryEvidenceIdentity) -> Self {
        Self::Query(identity)
    }
}

impl From<BridgeIdentityEvidence> for RawBasisIdentity {
    fn from(identity: BridgeIdentityEvidence) -> Self {
        Self::Bridge(identity)
    }
}
