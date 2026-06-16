use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{ForgeQueryMutationAuthorityIdentity, ForgeQueryMutationSymbolIdentity};
use forge_runtime_bridge::facade::{
    bridge_truth_external_identity_token, BridgeContinuityAuthoritativeIdentity,
    BridgeExistingTruthBindingAuthoritativeIdentity, BridgeIdentityEvidence,
    BridgeNamingAttachmentIdentity, BridgeNamingAuthoritativeIdentity,
    BridgeSymbolicTargetSymbolIdentity,
};

impl ForgeQueryMutationAuthorityIdentity {
    pub(in crate::runtime) fn from_bridge_existing_truth_authority(
        role: &'static str,
        identity: &BridgeExistingTruthBindingAuthoritativeIdentity,
    ) -> Self {
        Self::from_bridge_authority(role, "existing-truth", identity.as_str())
    }

    pub(in crate::runtime) fn from_bridge_naming_attachment(
        role: &'static str,
        identity: &BridgeNamingAttachmentIdentity,
    ) -> Self {
        Self::from_bridge_authority(role, "naming-attachment", identity.as_str())
    }

    pub(in crate::runtime) fn from_bridge_naming_authority(
        role: &'static str,
        identity: &BridgeNamingAuthoritativeIdentity,
    ) -> Self {
        Self::from_bridge_authority(role, "naming-authority", identity.as_str())
    }

    pub(in crate::runtime) fn from_bridge_continuity_authority(
        role: &'static str,
        identity: &BridgeContinuityAuthoritativeIdentity,
    ) -> Self {
        Self::from_bridge_authority(role, "continuity-authority", identity.as_str())
    }

    fn from_bridge_authority(
        role: &'static str,
        bridge_family: &'static str,
        authority_label: &str,
    ) -> Self {
        let bridge_evidence = bridge_evidence_from_authority_label(authority_label);
        Self {
            label: authority_label.to_string(),
            identity: bridge_import_identity(role, bridge_family, &bridge_evidence),
        }
    }
}

impl ForgeQueryMutationSymbolIdentity {
    pub(in crate::runtime) fn from_bridge_symbolic_target(
        role: &'static str,
        identity: &BridgeSymbolicTargetSymbolIdentity,
    ) -> Self {
        let bridge_evidence = bridge_evidence_from_authority_label(identity.as_str());
        Self {
            label: identity.as_str().to_string(),
            identity: bridge_import_identity(role, "symbolic-target-symbol", &bridge_evidence),
        }
    }
}

fn bridge_evidence_from_authority_label(label: &str) -> BridgeIdentityEvidence {
    BridgeIdentityEvidence::from_external_authority(bridge_truth_external_identity_token(label))
}

fn bridge_import_identity(
    role: &'static str,
    bridge_family: &'static str,
    identity: &BridgeIdentityEvidence,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAuthorityIdentity)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_shape(ForgeQueryEvidenceTag::new("bridge_family"), bridge_family)
        .field_bridge_retained_evidence_identity(
            ForgeQueryEvidenceTag::new("bridge_identity"),
            identity,
        )
        .seal()
}
