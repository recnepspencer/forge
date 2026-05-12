use super::super::*;

pub(in crate::runtime::tests) fn geometry_relation_binding(
    authoritative_identity: &str,
    resolved_entity_identity: &str,
    target_collection: &str,
) -> ForgeQueryExistingTruthTargetBinding {
    let target =
        ForgeQueryExistingRelationTarget::new(authoritative_identity, resolved_entity_identity)
            .expect("existing relation target should build")
            .in_target_collection(target_collection)
            .expect("existing relation target collection should build");
    ForgeQueryExistingTruthTargetBinding::from_relation_target(target)
        .expect("relation binding should build")
}
