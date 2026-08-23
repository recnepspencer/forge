pub(crate) fn presentation_bridge_registrations() -> Vec<(
    worth_runtime_bridge::facade::BridgeMappingRegistration,
    worth_runtime_bridge::facade::BridgeAspectRegistration,
)> {
    use worth_runtime_bridge::facade::*;

    super::installed_operation::contracts::FIELDS
        .into_iter()
        .zip(super::presentation_aspect_contracts())
        .map(|((aspect, _, _, _), contract)| {
            let scope = TruthPatchScope::new(
                MappingSelector::any(),
                AspectKeySelector::exact(contract.key().clone()),
                TruthPatchTargetSelector::AuthoritativeAspect,
            );
            let mapping = BridgeMappingRegistration::new(
                BridgeMappingId::from_stable_name(aspect),
                scope.clone(),
                SnapshotReadContract::new(contract),
                SignalInvalidationScope::from_stable_name(aspect),
                CoarseRoutingMode::Direct,
            );
            let aspect_mapping = BridgeAspectRegistration::new(
                BridgeAspectRegistrationId::from_stable_name(aspect),
                scope,
                mapping.snapshot_read_contract().clone(),
                TruthDeltaSurfaceKind::AuthoritativeAspect,
                SubscriptionSliceKind::SignalPartition,
                SliceWideningPolicy::RegisteredPartitionWidening,
            );
            (mapping, aspect_mapping)
        })
        .collect()
}
