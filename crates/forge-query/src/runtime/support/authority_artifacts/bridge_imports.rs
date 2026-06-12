use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{ForgeQueryMutationAuthorityIdentity, ForgeQueryMutationSymbolIdentity};
use forge_runtime_bridge::facade::{
    BridgeContinuityAuthoritativeIdentity, BridgeExistingTruthBindingAuthoritativeIdentity,
    BridgeIdentityEvidence, BridgeNamingAttachmentIdentity, BridgeNamingAuthoritativeIdentity,
    BridgeSymbolicTargetSymbolIdentity,
};

impl ForgeQueryMutationAuthorityIdentity {
    pub(in crate::runtime) fn from_bridge_existing_truth_authority(
        role: &'static str,
        identity: &BridgeExistingTruthBindingAuthoritativeIdentity,
    ) -> Self {
        Self::from_bridge_authority(role, "existing-truth", identity.evidence_identity())
    }

    pub(in crate::runtime) fn from_bridge_naming_attachment(
        role: &'static str,
        identity: &BridgeNamingAttachmentIdentity,
    ) -> Self {
        Self::from_bridge_authority(role, "naming-attachment", identity.evidence_identity())
    }

    pub(in crate::runtime) fn from_bridge_naming_authority(
        role: &'static str,
        identity: &BridgeNamingAuthoritativeIdentity,
    ) -> Self {
        Self::from_bridge_authority(role, "naming-authority", identity.evidence_identity())
    }

    pub(in crate::runtime) fn from_bridge_continuity_authority(
        role: &'static str,
        identity: &BridgeContinuityAuthoritativeIdentity,
    ) -> Self {
        Self::from_bridge_authority(role, "continuity-authority", identity.evidence_identity())
    }

    fn from_bridge_authority(
        role: &'static str,
        bridge_family: &'static str,
        identity: BridgeIdentityEvidence,
    ) -> Self {
        Self {
            label: identity.as_str().to_string(),
            identity: bridge_import_identity(role, bridge_family, &identity),
        }
    }
}

impl ForgeQueryMutationSymbolIdentity {
    pub(in crate::runtime) fn from_bridge_symbolic_target(
        role: &'static str,
        identity: &BridgeSymbolicTargetSymbolIdentity,
    ) -> Self {
        let identity = identity.evidence_identity();
        Self {
            label: identity.as_str().to_string(),
            identity: bridge_import_identity(role, "symbolic-target-symbol", &identity),
        }
    }
}

fn bridge_import_identity(
    role: &'static str,
    bridge_family: &'static str,
    identity: &BridgeIdentityEvidence,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAuthorityIdentity)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_shape(ForgeQueryEvidenceTag::new("bridge_family"), bridge_family)
        .field_identity(ForgeQueryEvidenceTag::new("bridge_identity"), identity)
        .seal()
}
