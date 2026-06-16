use forge_query::facade::ForgeQueryEntityIdentity;
use forge_runtime_bridge::facade::{
    bridge_identity_reporting_label, BridgeIdentityEvidence, RelationalBridgeRecordIdentityKind,
};

pub(crate) fn query_entity_identity_reporting_label(
    identity: &ForgeQueryEntityIdentity,
) -> String {
    if let Some(parts) = identity.relational_record_parts() {
        let kind = match parts.kind() {
            RelationalBridgeRecordIdentityKind::Entity => "entity",
            RelationalBridgeRecordIdentityKind::Relation => "relation",
        };
        return format!(
            "{kind}:{}:{}:{}",
            parts.partition_id(),
            parts.local_slot(),
            parts.generation()
        );
    }
    "non-relational-query-entity-identity".to_string()
}

pub(crate) fn bridge_identity_projection(evidence: BridgeIdentityEvidence) -> String {
    bridge_identity_reporting_label(&evidence).to_string()
}
