use crate::runtime::tests::source_ingress_test_support::{empty_artifact, runtime_from_artifact};
use crate::runtime::{
    WorthUiSourceIngressDenialReason, WorthUiSourceProvider,
    WorthUiWatchedCandidateSubmissionDenial, WorthUiWatcherEvent,
};

pub(crate) fn lower_file_submission<const N: usize>(
    provider: WorthUiSourceProvider,
    events: [WorthUiWatcherEvent; N],
    snapshot: &crate::capability::CapabilitySnapshot,
) -> crate::runtime::WorthUiWatchedCandidateSubmission {
    let mut session = runtime_from_artifact(empty_artifact())
        .source_ingress(provider)
        .start();
    session
        .ingest(events)
        .expect("events debounce")
        .lower_to_candidate_submission(snapshot)
        .expect("candidate submission lowers")
}

pub(crate) fn lower_rust_submission<const N: usize>(
    provider: WorthUiSourceProvider,
    events: [WorthUiWatcherEvent; N],
    snapshot: &crate::capability::CapabilitySnapshot,
) -> crate::runtime::WorthUiWatchedCandidateSubmission {
    lower_file_submission(provider, events, snapshot)
}

pub(crate) fn assert_source_denial_reason(
    denial: WorthUiWatchedCandidateSubmissionDenial,
    expected_reason: WorthUiSourceIngressDenialReason,
) {
    match denial {
        WorthUiWatchedCandidateSubmissionDenial::SourceIngress(source_denial) => {
            assert_eq!(source_denial.reason(), expected_reason);
        }
        WorthUiWatchedCandidateSubmissionDenial::Candidate(candidate_denial) => {
            panic!("expected source ingress denial, got {candidate_denial:?}");
        }
    }
}

pub(crate) fn source_backed_package_component(id: &str) -> crate::capability::ComponentDescriptor {
    crate::capability::ComponentDescriptor::new(
        crate::capability::ComponentId::new(id).unwrap(),
        crate::capability::ComponentPropSchema::named(format!("{id}.props")),
        crate::capability::ComponentChildPolicy::no_children(),
        crate::capability::ComponentStateOwnership::runtime_owned(),
    )
}

pub(crate) fn source_backed_package_region() -> crate::capability::MosaicRegionKindDescriptor {
    crate::capability::MosaicRegionKindDescriptor::new(
        crate::capability::MosaicRegionKindId::new("workspace.region.primary").unwrap(),
        crate::capability::MosaicRegionRole::primary(),
    )
    .with_sizing_behavior(crate::capability::MosaicSizingBehavior::fills_available_space())
    .with_scroll_ownership(crate::capability::MosaicScrollOwnership::region_owned())
    .with_focus_scope(crate::capability::MosaicFocusScopeKind::active_surface_scope())
    .with_child_rule(crate::capability::MosaicChildRule::accepts_surfaces())
    .with_allowed_surface_class(crate::capability::SurfacePlacementClass::primary_region())
    .with_persistence(crate::capability::MosaicRegionPersistence::restorable())
    .with_clipping(crate::capability::MosaicClippingPosture::clip_to_region())
    .with_hit_test(crate::capability::MosaicHitTestPosture::participates())
}

pub(crate) fn source_backed_package_sizing() -> crate::capability::MosaicSizingContractDescriptor {
    crate::capability::MosaicSizingContractDescriptor::new(
        crate::capability::MosaicSizingContractId::new("workspace.sizing.mosaic_support").unwrap(),
        crate::capability::MosaicSizingKind::fill(),
    )
    .with_measurement_authority(crate::capability::MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(crate::capability::MosaicResizePermission::user_resizable())
    .with_persistence(crate::capability::MosaicSizingPersistence::restorable())
    .with_overflow_behavior(crate::capability::MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(
        crate::capability::MosaicParentGrowthBehavior::does_not_force_parent(),
    )
    .with_viewport_constraint(crate::capability::MosaicViewportConstraint::clamp_to_viewport())
    .with_named_measurement(crate::capability::NamedMeasurementDefinition::new(
        crate::capability::NamedMeasurementToken::new("workspace.measurement.mosaic_support")
            .unwrap(),
        crate::capability::MeasurementValue::logical_pixels(320),
        crate::capability::MeasurementConstraint::between(
            crate::capability::MeasurementValue::logical_pixels(200),
            crate::capability::MeasurementValue::logical_pixels(640),
        ),
    ))
}
