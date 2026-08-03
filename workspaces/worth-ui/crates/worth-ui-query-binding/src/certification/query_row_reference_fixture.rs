pub fn query_row_reference_fixture(local_slot: u64) -> crate::UiCollectionProjectionRowReference {
    let entity =
        worth_query::facade::foundation::WorthQueryEntityIdentity::from_bridge_record_projection(
            worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(
                0, local_slot, 0,
            ),
        );
    crate::UiCollectionProjectionRowReference::query_issued(entity.evidence_identity())
}
