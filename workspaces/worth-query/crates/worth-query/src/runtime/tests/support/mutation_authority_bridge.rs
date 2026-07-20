use crate::runtime::WorthQueryMutationAuthorityIdentity;
use worth_runtime_bridge::facade::{
    BridgeContinuityAuthoritativeIdentity, BridgeNamingAttachmentIdentity,
    BridgeNamingAuthoritativeIdentity,
};

pub(in crate::runtime::tests) fn expected_bridge_naming_attachment_label(
    authority: &WorthQueryMutationAuthorityIdentity,
) -> String {
    BridgeNamingAttachmentIdentity::from_bridge_evidence(
        &authority.evidence_identity().bridge_evidence_identity(),
    )
    .as_str()
    .to_string()
}

pub(in crate::runtime::tests) fn expected_bridge_naming_authority_label(
    authority: &WorthQueryMutationAuthorityIdentity,
) -> String {
    BridgeNamingAuthoritativeIdentity::from_bridge_evidence(
        &authority.evidence_identity().bridge_evidence_identity(),
    )
    .as_str()
    .to_string()
}

pub(in crate::runtime::tests) fn expected_bridge_continuity_authority_label(
    authority: &WorthQueryMutationAuthorityIdentity,
) -> String {
    BridgeContinuityAuthoritativeIdentity::from_bridge_evidence(
        &authority.evidence_identity().bridge_evidence_identity(),
    )
    .as_str()
    .to_string()
}
