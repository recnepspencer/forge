use crate::runtime::ForgeQueryMutationAuthorityIdentity;
use forge_runtime_bridge::facade::{
    BridgeContinuityAuthoritativeIdentity, BridgeExistingTruthBindingAuthoritativeIdentity,
    BridgeNamingAttachmentIdentity, BridgeNamingAuthoritativeIdentity,
};

pub(in crate::runtime::tests) fn expected_bridge_naming_attachment_label(
    authority: &ForgeQueryMutationAuthorityIdentity,
) -> String {
    BridgeNamingAttachmentIdentity::from_bridge_evidence(
        &authority.evidence_identity().bridge_evidence_identity(),
    )
    .as_str()
    .to_string()
}

pub(in crate::runtime::tests) fn expected_bridge_naming_authority_label(
    authority: &ForgeQueryMutationAuthorityIdentity,
) -> String {
    BridgeNamingAuthoritativeIdentity::from_bridge_evidence(
        &authority.evidence_identity().bridge_evidence_identity(),
    )
    .as_str()
    .to_string()
}

pub(in crate::runtime::tests) fn expected_bridge_continuity_authority_label(
    authority: &ForgeQueryMutationAuthorityIdentity,
) -> String {
    BridgeContinuityAuthoritativeIdentity::from_bridge_evidence(
        &authority.evidence_identity().bridge_evidence_identity(),
    )
    .as_str()
    .to_string()
}

#[allow(dead_code)]
pub(in crate::runtime::tests) fn expected_bridge_existing_truth_authority_label(
    authority: &ForgeQueryMutationAuthorityIdentity,
) -> String {
    BridgeExistingTruthBindingAuthoritativeIdentity::from_bridge_evidence(
        &authority.evidence_identity().bridge_evidence_identity(),
    )
    .as_str()
    .to_string()
}
