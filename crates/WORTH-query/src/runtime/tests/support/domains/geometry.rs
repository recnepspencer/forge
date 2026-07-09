use super::super::*;
use worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts;

pub(in crate::runtime::tests) fn geometry_relation_binding(
    authoritative_identity: &str,
    resolved_entity_identity: &str,
    target_collection: &str,
) -> WorthQueryExistingTruthTargetBinding {
    let local_slot = resolved_entity_identity
        .rsplit_once(':')
        .and_then(|(_, suffix)| suffix.parse::<u64>().ok())
        .unwrap_or(1);
    let resolved_relation_identity = WorthQueryEntityIdentity::from_relational_record(
        RelationalBridgeRecordIdentityParts::relation(2, local_slot, 0),
    );
    let target = WorthQueryExistingRelationTarget::new(
        crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(
            crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new(
                authoritative_identity,
            )
            .expect("existing-truth authority label"),
        )
        .expect("existing-truth authority identity"),
        resolved_relation_identity,
    )
    .expect("existing relation target should build")
    .in_target_collection(target_collection)
    .expect("existing relation target collection should build");
    WorthQueryExistingTruthTargetBinding::from_relation_target(target)
        .expect("relation binding should build")
}
