use super::*;

pub(in crate::harness::milestone_eight_certification) fn aspect_value(
    value: AspectValue,
) -> AspectValue {
    encode_snapshot_aspect_read_value(&value)
}

pub(in crate::harness::milestone_eight_certification) fn native_grouped_patch_envelope(
    commit_identity: TruthCommitIdentity,
    patch_identity: TruthPatchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    branch_identity: TruthBranchIdentity,
) -> BridgeCommittedPatchEnvelope {
    BridgeCommittedPatchEnvelope::new(
        BridgeCommittedPatchEnvelopeIdentity::new(
            commit_identity,
            patch_identity,
            snapshot_identity,
            branch_identity,
        ),
        vec![BridgeCommittedPatchItem::with_relational_record_target(
            milestone_eight_record_parts("task-1"),
            BridgeCommittedPatchTarget::entity_field_path(
                AspectLocator::new(LocatorAuthority::Authoritative, aspect_key("status")),
                CanonicalFieldPath::single(field_key("lane")),
            ),
        )],
    )
    .expect("milestone eight native grouped patch envelope should construct")
}

pub(in crate::harness::milestone_eight_certification) fn string_snapshot_read(
    record_parts: RelationalBridgeRecordIdentityParts,
    aspect: &str,
) -> SnapshotReadRequest {
    SnapshotReadRequest::for_relational_record(
        record_parts,
        SnapshotReadContract::scalar(aspect_key(aspect), ScalarAspectType::String),
    )
}

pub(in crate::harness::milestone_eight_certification) fn relational_grouped_projection_contract(
    grouping_aspect: &str,
    identity_binding_aspect: &str,
    grouping_binding_aspect: &str,
) -> RelationalGroupedProjectionContract {
    RelationalGroupedProjectionContract::new(
        aspect_key(grouping_aspect),
        aspect_key(identity_binding_aspect),
        aspect_key(grouping_binding_aspect),
    )
}

pub(in crate::harness::milestone_eight_certification) fn aspect_key(label: &str) -> AspectKey {
    AspectKey::new(label).expect("certification grouped projection aspect key must be foundational")
}

pub(in crate::harness::milestone_eight_certification) fn field_key(label: &str) -> FieldKey {
    FieldKey::new(label.to_owned())
        .expect("certification grouped projection field key must be foundational")
}
