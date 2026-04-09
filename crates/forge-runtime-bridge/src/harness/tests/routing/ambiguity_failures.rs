use crate::facade::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeBuildErrorKind, MappingSelector,
    RuntimeBridgeBuilder, SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind,
    TruthPatchScope,
};

use super::super::support::registration;
use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};

#[test]
fn ambiguous_slice_registration_fails_explicitly() {
    let source = InMemoryRelationalBridgeSource::default();

    let error = RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .register_mapping(registration())
        .register_aspect_mapping(BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::new("entity-wide"),
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::any(),
                MappingSelector::exact("name"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        ))
        .register_aspect_mapping(BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::new("aspect-wide"),
            TruthPatchScope::new(
                MappingSelector::any(),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        ))
        .build()
        .expect_err("ambiguous aspect registrations must fail at freeze time");

    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::AmbiguousAspectRegistration
    );
}
