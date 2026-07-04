use worth_ui::facade::declaration::{
    UiDeclarationSupportMilestoneExpectation, UiDeclarationSupportRowSchemaKind,
    UiDeclarationUnsupportedPosture,
};
use worth_ui::facade::graph::{
    UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchOriginClass,
    UiGraphTouchRuntimeLane, UiGraphTouchTargetClass, UiGraphTouchTiming,
};
use worth_ui::facade::obligations::{
    UiObligationCheckKind, UiObligationFamily, UiObligationSelectionReason,
    UiObligationSupportBasis, UiObligationSupportSelectionPosture, UiObligationWorldProfileClass,
};
use worth_ui_runtime::facade::{runtime_origin_fixture, WorthUiTouchOriginFixtureVariant};

#[test]
fn host_observation_selection_is_stable_and_keeps_measurement_and_host_requirement_separate() {
    let fixture = runtime_origin_fixture(WorthUiTouchOriginFixtureVariant::Baseline);

    assert_eq!(
        fixture
            .control_artifact()
            .support_snapshot()
            .expect("control support snapshot should admit")
            .row(UiDeclarationSupportRowSchemaKind::HostCapability)
            .expect("host capability row should exist")
            .unsupported_posture(),
        Some(
            UiDeclarationUnsupportedPosture::ArchitecturallyOwnedButNotYetAdmitted {
                expected_in: UiDeclarationSupportMilestoneExpectation::Milestone32,
            },
        )
    );

    // Host-observation receipts still require aligned active-runtime inspection.
    // This runtime-origin fixture is the narrowest honest production path available here.
    let touch = fixture
        .app
        .graph()
        .touches()
        .from_node(
            fixture
                .app
                .graph()
                .touches()
                .host_observation_receipt(fixture.runtime.inspect_active(), &fixture.inspection)
                .expect("host observation should admit"),
            UiGraphTouchTiming::ReactiveObservation,
            fixture.control_graph_node_identity(),
            UiGraphTouchAspects::new()
                .measurement(UiGraphTouchAspectPosture::Read)
                .host_capability(UiGraphTouchAspectPosture::Read),
        )
        .expect("host touch should admit");

    let left = fixture.app.admission().select_obligations(&touch);
    let right = fixture.app.admission().select_obligations(&touch);

    assert_eq!(left, right);
    assert_eq!(
        touch.origin().class(),
        UiGraphTouchOriginClass::HostObservation
    );
    assert_eq!(
        touch
            .aspects()
            .iter()
            .map(|fact| fact.lane())
            .collect::<Vec<_>>(),
        vec![
            UiGraphTouchRuntimeLane::Measurement,
            UiGraphTouchRuntimeLane::HostCapability,
        ]
    );
    assert_eq!(
        left.obligations()
            .iter()
            .map(|obligation| obligation.family())
            .collect::<Vec<_>>(),
        vec![
            UiObligationFamily::MeasurementRequirement,
            UiObligationFamily::HostCapabilityRequirement,
        ]
    );

    let measurement = &left.obligations()[0];
    assert_eq!(
        measurement.identity().obligation_family(),
        UiObligationFamily::MeasurementRequirement
    );
    assert_eq!(
        measurement.identity().support_basis(),
        UiObligationSupportBasis::MeasurementPolicy
    );
    assert_eq!(
        measurement.identity().aspect_scope(),
        &[UiGraphTouchRuntimeLane::Measurement]
    );
    assert_eq!(measurement.identity().world(), touch.world());
    assert_eq!(
        measurement.identity().target().class(),
        UiGraphTouchTargetClass::Node
    );
    assert_eq!(
        measurement.check_kind(),
        UiObligationCheckKind::PrerequisiteRequirement
    );
    assert_eq!(
        measurement.selection_reasons(),
        [
            UiObligationSelectionReason::TouchTargetClass(UiGraphTouchTargetClass::Node),
            UiObligationSelectionReason::TouchOriginClass(UiGraphTouchOriginClass::HostObservation),
            UiObligationSelectionReason::WorldProfile(UiObligationWorldProfileClass::Authoritative,),
            UiObligationSelectionReason::SupportPosture(
                UiObligationSupportSelectionPosture::Supported,
            ),
            UiObligationSelectionReason::SupportRow(
                UiDeclarationSupportRowSchemaKind::MeasurementPolicy,
            ),
            UiObligationSelectionReason::TouchRuntimeLane(UiGraphTouchRuntimeLane::Measurement),
            UiObligationSelectionReason::TouchAspectPosture(UiGraphTouchAspectPosture::Read),
        ]
    );

    let host_requirement = &left.obligations()[1];
    assert_eq!(
        host_requirement.identity().obligation_family(),
        UiObligationFamily::HostCapabilityRequirement
    );
    assert_eq!(
        host_requirement.identity().support_basis(),
        UiObligationSupportBasis::HostCapability
    );
    assert_eq!(
        host_requirement.identity().aspect_scope(),
        &[UiGraphTouchRuntimeLane::HostCapability]
    );
    assert_eq!(
        host_requirement.check_kind(),
        UiObligationCheckKind::CapabilityGapScreen
    );
    assert_eq!(
        host_requirement.selection_reasons(),
        [
            UiObligationSelectionReason::TouchTargetClass(UiGraphTouchTargetClass::Node),
            UiObligationSelectionReason::TouchOriginClass(UiGraphTouchOriginClass::HostObservation),
            UiObligationSelectionReason::WorldProfile(UiObligationWorldProfileClass::Authoritative,),
            UiObligationSelectionReason::SupportPosture(
                UiObligationSupportSelectionPosture::Deferred,
            ),
            UiObligationSelectionReason::SupportRow(
                UiDeclarationSupportRowSchemaKind::HostCapability,
            ),
            UiObligationSelectionReason::TouchRuntimeLane(UiGraphTouchRuntimeLane::HostCapability),
            UiObligationSelectionReason::TouchAspectPosture(UiGraphTouchAspectPosture::Read),
        ]
    );
}
