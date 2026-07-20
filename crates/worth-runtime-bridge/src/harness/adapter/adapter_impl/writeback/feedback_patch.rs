pub(in crate::harness::adapter::adapter_impl::writeback) fn bridge_feedback_patch(
    envelope_identity: crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity,
    feedback_context: &crate::facade::BridgeWritebackFeedbackContext,
) -> crate::facade::BridgeCommittedPatchEnvelope {
    crate::facade::BridgeCommittedPatchEnvelope::new(
        crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            crate::facade::BridgeProducerMetadata::bridge_harness_fixture()
                .with_writeback_feedback_context(feedback_context.clone()),
            envelope_identity.commit_identity().clone(),
            envelope_identity.patch_identity().clone(),
            envelope_identity.snapshot_identity().clone(),
            envelope_identity.branch_identity().clone(),
        ),
        vec![crate::facade::BridgeCommittedPatchItem::with_target(
            "user",
            crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                worth_foundational::facade::AspectLocator::new(
                    worth_foundational::facade::LocatorAuthority::Authoritative,
                    worth_foundational::facade::AspectKey::new("profile")
                        .expect("valid bridge patch aspect key"),
                ),
                worth_foundational::facade::CanonicalFieldPath::single(
                    worth_foundational::facade::FieldKey::new("name".to_owned())
                        .expect("valid foundational field key"),
                ),
            ),
        )],
    )
    .expect("writeback feedback committed patch envelope should construct")
}

pub(in crate::harness::adapter::adapter_impl::writeback) fn feedback_context_hint(
    patch: &crate::facade::BridgeCommittedPatchEnvelope,
) -> Option<&crate::facade::BridgeWritebackFeedbackContext> {
    patch.producer_metadata().writeback_feedback_context()
}
