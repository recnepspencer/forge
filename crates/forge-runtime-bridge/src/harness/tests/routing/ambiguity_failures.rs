use crate::facade::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeBuildErrorKind, MappingSelector,
    RuntimeBridgeBuilder, SliceWideningPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind,
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
            BridgeAspectRegistrationId::admit_bridge_owned("entity-wide"),
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                crate::facade::AspectKeySelector::exact(
                    forge_foundational::facade::AspectKey::new("profile")
                        .expect("valid native aspect key"),
                ),
                crate::facade::TruthPatchTargetSelector::any(),
            ),
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ))
        .register_aspect_mapping(BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::admit_bridge_owned("aspect-wide"),
            TruthPatchScope::for_entity_field(
                MappingSelector::any(),
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid native field key"),
            ),
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ))
        .build()
        .expect_err("ambiguous aspect registrations must fail at freeze time");

    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::AmbiguousAspectRegistration
    );
}
