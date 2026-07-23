use super::*;

pub(crate) fn field_change_envelope_with_width(
    matching_changes: usize,
    unrelated_items: usize,
) -> BridgeCommittedPatchEnvelope {
    let binding = AspectBinding::EntityField {
        field: FieldKey::new("profile").unwrap(),
    };
    let mut items = (0..matching_changes)
        .map(|_| {
            BridgeCommittedPatchItem::with_relational_semantic_change(
                RelationalBridgeRecordIdentityParts::entity(0, 1, 1),
                BridgeCommittedPatchTarget::entity_field_path(
                    AspectLocator::new(LocatorAuthority::Authoritative, aspect_key()),
                    field_path(),
                ),
                BridgeSemanticAspectChange::from_authoritative_publication(
                    aspect_key(),
                    AspectIdentity(31),
                    AspectContractRevision(4),
                    binding.clone(),
                    AuthoritativeAspectChangeKind::FieldSet,
                    Some(field_path()),
                ),
            )
        })
        .collect::<Vec<_>>();
    items.extend((0..unrelated_items).map(|index| {
        BridgeCommittedPatchItem::with_target(
            format!("unrelated:{index}"),
            BridgeCommittedPatchTarget::authoritative_aspect(AspectLocator::new(
                LocatorAuthority::Authoritative,
                aspect_key(),
            )),
        )
    }));
    BridgeCommittedPatchEnvelope::new(
        BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            BridgeProducerMetadata::bridge_harness_fixture(),
            truth_commit(40),
            truth_patch(40),
            truth_snapshot(40, 40),
            truth_branch("main"),
        ),
        items,
    )
    .expect("multi-change semantic envelope is canonical")
}
